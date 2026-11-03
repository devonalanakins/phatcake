//! Sample-time control delivery for Phatcake's four-deck PCM engine.
//!
//! A bounded wait-free SPSC ring is allocated before the audio stream starts.
//! Only one producer sends messages. Only one audio thread consumes them.
//! Each event is applied before its requested absolute output frame, when
//! already available to the consumer; late arrivals apply at the next poll.
//!
//! This is not a touch-input or physical-latency implementation.

#![forbid(unsafe_code)]

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use phatcake_engine::{Deck, Engine, EngineError, StereoRenderer};
use rtrb::{Consumer, PopError, Producer, PushError, RingBuffer};

/// Maximum events placed into the ring before the sender gets backpressure.
pub const COMMAND_CAPACITY: usize = 1024;

/// Maximum events applied during a single render call.
pub const MAX_EVENTS_PER_RENDER: usize = 64;

/// Maximum rendering interval between checks for newly arrived commands.
pub const POLL_FRAMES: usize = 32;

/// A deck identifier guaranteed to address one of the four existing decks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeckId {
    A,
    B,
    C,
    D,
}

impl DeckId {
    fn index(self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
            Self::D => 3,
        }
    }
}

/// A compact, copyable control payload with no owned heap data.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ControlChange {
    Play(DeckId),
    Pause(DeckId),
    Stop(DeckId),
    Seek { deck: DeckId, position: f64 },
    SetRate { deck: DeckId, rate: f64 },
    SetGain { deck: DeckId, gain: f32 },
    SetCue { deck: DeckId, enabled: bool },
    SetLoop { deck: DeckId, start: usize, end: usize },
    ClearLoop(DeckId),
    SetCrossfader(f32),
}

/// Timestamp is the absolute frame index of the audio rendering clock.
/// The event takes effect before sample `at_frame` is rendered.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Command {
    pub at_frame: u64,
    pub change: ControlChange,
}

/// No command is silently overwritten when the bounded queue is full.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SendError {
    Full(Command),
    OutOfOrder { previous: u64, requested: u64 },
}

/// Metrics describe sample-time scheduling, not physical touch-to-sound delay.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ControlStats {
    pub applied: u64,
    pub invalid: u64,
    pub late: u64,
    pub deferred: u64,
}

/// A single-producer handle; own it on the UI/event coordination thread.
pub struct ControlSender {
    producer: Producer<Command>,
    published_frame: Arc<AtomicU64>,
    last_queued: Option<u64>,
}

impl ControlSender {
    /// Snapshot of the audio engine's output-frame position.
    /// This is NOT a wall-clock timestamp or DAC playback position.
    pub fn current_frame(&self) -> u64 {
        self.published_frame.load(Ordering::Acquire)
    }

    /// Queue timestamps in nondecreasing order. Equal-frame commands retain
    /// FIFO ordering. On full/out-of-order errors, queue state is unchanged.
    pub fn send(&mut self, command: Command) -> Result<(), SendError> {
        if let Some(previous) = self.last_queued {
            if command.at_frame < previous {
                return Err(SendError::OutOfOrder {
                    previous,
                    requested: command.at_frame,
                });
            }
        }
        match self.producer.push(command) {
            Ok(()) => {
                self.last_queued = Some(command.at_frame);
                Ok(())
            }
            Err(PushError::Full(event)) => Err(SendError::Full(event)),
        }
    }

    /// Schedule a command against the published output-rendering frame clock.
    /// Monotonic ordering still applies; urgent input cannot jump ahead of
    /// previously queued future automation on this first single lane.
    pub fn send_after(
        &mut self,
        lead_frames: u64,
        change: ControlChange,
    ) -> Result<(), SendError> {
        let at_frame = self.current_frame().saturating_add(lead_frames);
        self.send(Command { at_frame, change })
    }
}

/// Owns four decks and the sole queue consumer on the audio callback thread.
pub struct ControlledEngine {
    engine: Engine,
    consumer: Consumer<Command>,
    pending: Option<Command>,
    frame: u64,
    published_frame: Arc<AtomicU64>,
    stats: ControlStats,
}

/// Allocate a bounded queue once, before audio playback begins.
pub fn control_channel(engine: Engine) -> (ControlSender, ControlledEngine) {
    let (producer, consumer) = RingBuffer::new(COMMAND_CAPACITY);
    let published_frame = Arc::new(AtomicU64::new(0));
    (
        ControlSender {
            producer,
            published_frame: Arc::clone(&published_frame),
            last_queued: None,
        },
        ControlledEngine {
            engine,
            consumer,
            pending: None,
            frame: 0,
            published_frame,
            stats: ControlStats::default(),
        },
    )
}

impl ControlledEngine {
    pub fn current_frame(&self) -> u64 {
        self.frame
    }

    pub fn stats(&self) -> ControlStats {
        self.stats
    }

    /// Read-only inspection; loading and other non-RT edits must be done
    /// before constructing the channel, not during live rendering.
    pub fn deck(&self, deck: DeckId) -> &Deck {
        self.engine
            .deck(deck.index())
            .expect("DeckId is always one of four valid deck indices")
    }

    pub fn render_into(
        &mut self,
        master: &mut [f32],
        cue: &mut [f32],
    ) -> Result<(), EngineError> {
        <Self as StereoRenderer>::render_into(self, master, cue)
    }

    /// Process at most `budget` due events per render invocation.
    fn apply_due(&mut self, budget: &mut usize) {
        while *budget > 0 {
            let event = match self.pending.take().or_else(|| self.consumer.pop().ok()) {
                Some(event) => event,
                None => return,
            };
            if event.at_frame > self.frame {
                self.pending = Some(event);
                return;
            }

            *budget -= 1;
            if event.at_frame < self.frame {
                self.stats.late = self.stats.late.saturating_add(1);
            }

            if apply_change(&mut self.engine, event.change).is_ok() {
                self.stats.applied = self.stats.applied.saturating_add(1);
            } else {
                self.stats.invalid = self.stats.invalid.saturating_add(1);
            }
        }
    }
}

fn apply_change(engine: &mut Engine, change: ControlChange) -> Result<(), EngineError> {
    match change {
        ControlChange::Play(deck) => engine
            .deck_mut(deck.index())
            .expect("valid DeckId")
            .play(),
        ControlChange::Pause(deck) => {
            engine.deck_mut(deck.index()).expect("valid DeckId").pause();
            Ok(())
        }
        ControlChange::Stop(deck) => {
            engine.deck_mut(deck.index()).expect("valid DeckId").stop();
            Ok(())
        }
        ControlChange::Seek { deck, position } => engine
            .deck_mut(deck.index())
            .expect("valid DeckId")
            .seek(position),
        ControlChange::SetRate { deck, rate } => engine
            .deck_mut(deck.index())
            .expect("valid DeckId")
            .set_playback_rate(rate),
        ControlChange::SetGain { deck, gain } => engine
            .deck_mut(deck.index())
            .expect("valid DeckId")
            .set_gain(gain),
        ControlChange::SetCue { deck, enabled } => {
            engine
                .deck_mut(deck.index())
                .expect("valid DeckId")
                .set_cue_enabled(enabled);
            Ok(())
        }
        ControlChange::SetLoop { deck, start, end } => engine
            .deck_mut(deck.index())
            .expect("valid DeckId")
            .set_loop(start, end),
        ControlChange::ClearLoop(deck) => {
            engine
                .deck_mut(deck.index())
                .expect("valid DeckId")
                .clear_loop();
            Ok(())
        }
        ControlChange::SetCrossfader(value) => engine.set_crossfader(value),
    }
}

impl StereoRenderer for ControlledEngine {
    fn render_into(
        &mut self,
        master: &mut [f32],
        cue: &mut [f32],
    ) -> Result<(), EngineError> {
        if master.len() != cue.len() || !master.len().is_multiple_of(2) {
            return Err(EngineError::InvalidOutputBuffers);
        }

        let total_frames = master.len() / 2;
        let mut processed = 0;
        let mut budget = MAX_EVENTS_PER_RENDER;

        while processed < total_frames {
            self.apply_due(&mut budget);
            let mut run = (total_frames - processed).min(POLL_FRAMES);

            if let Some(event) = self.pending {
                if event.at_frame > self.frame {
                    let until_event = event.at_frame - self.frame;
                    run = run.min(until_event as usize);
                }
            }

            // A budget-exhausted callback still advances audio. The next
            // callback can process deferred commands, marked as late then.
            if run == 0 {
                run = 1;
            }
            if budget == 0 {
                self.stats.deferred = self.stats.deferred.saturating_add(1);
            }

            let start = processed * 2;
            let end = (processed + run) * 2;
            self.engine.render_into(&mut master[start..end], &mut cue[start..end])?;
            self.frame = self.frame.saturating_add(run as u64);
            self.published_frame.store(self.frame, Ordering::Release);
            processed += run;
        }

        Ok(())
    }
}
