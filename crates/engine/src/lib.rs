//! Phatcake's deterministic, offline, four-deck audio foundation.
//!
//! This is NOT a hardware-backed low-latency DJ app. A single caller owns the
//! engine. Track preparation and all control changes occur outside rendering.
//! The render path allocates no memory, locks no mutex, and performs no I/O.

#![forbid(unsafe_code)]

/// An interleaved stereo source frame, normalized to [-1, 1].
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StereoFrame {
    pub left: f32,
    pub right: f32,
}

impl StereoFrame {
    pub const SILENCE: Self = Self {
        left: 0.0,
        right: 0.0,
    };

    pub const fn new(left: f32, right: f32) -> Self {
        Self { left, right }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrossfaderSide {
    Left,
    Right,
    Bypass,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineError {
    InvalidSampleRate,
    InvalidTrack,
    NoTrackLoaded,
    InvalidPosition,
    InvalidPlaybackRate,
    InvalidGain,
    InvalidLoop,
    InvalidCrossfader,
    InvalidOutputBuffers,
}

/// One independent PCM deck. Not safe to mutate concurrently with rendering.
#[derive(Debug)]
pub struct Deck {
    pcm: Vec<StereoFrame>,
    position: f64,
    playback_rate: f64,
    gain: f32,
    playing: bool,
    cue_enabled: bool,
    side: CrossfaderSide,
    loop_region: Option<(usize, usize)>,
}

impl Deck {
    fn new(side: CrossfaderSide) -> Self {
        Self {
            pcm: Vec::new(),
            position: 0.0,
            playback_rate: 1.0,
            gain: 1.0,
            playing: false,
            cue_enabled: false,
            side,
            loop_region: None,
        }
    }

    /// Loads decoded source PCM at exactly the engine's sample rate.
    /// This can allocate; do not call it from a real-time audio callback.
    pub fn load_pcm(&mut self, pcm: Vec<StereoFrame>) -> Result<(), EngineError> {
        if pcm.is_empty()
            || pcm
                .iter()
                .any(|frame| !frame.left.is_finite() || !frame.right.is_finite())
        {
            return Err(EngineError::InvalidTrack);
        }
        self.pcm = pcm;
        self.position = 0.0;
        self.playback_rate = 1.0;
        self.playing = false;
        self.loop_region = None;
        Ok(())
    }

    pub fn sample_count(&self) -> usize {
        self.pcm.len()
    }

    pub fn position_frames(&self) -> f64 {
        self.position
    }

    pub fn is_playing(&self) -> bool {
        self.playing
    }

    pub fn play(&mut self) -> Result<(), EngineError> {
        if self.pcm.is_empty() {
            return Err(EngineError::NoTrackLoaded);
        }
        if self.position < 0.0 || self.position >= self.pcm.len() as f64 {
            self.position = self.loop_region.map_or(0.0, |(start, _)| start as f64);
        }
        self.playing = true;
        Ok(())
    }

    pub fn pause(&mut self) {
        self.playing = false;
    }

    pub fn stop(&mut self) {
        self.playing = false;
        self.position = self.loop_region.map_or(0.0, |(start, _)| start as f64);
    }

    /// Position is in source frames and may be fractional.
    pub fn seek(&mut self, position: f64) -> Result<(), EngineError> {
        if !position.is_finite() || position < 0.0 || position >= self.pcm.len() as f64 {
            return Err(EngineError::InvalidPosition);
        }
        if let Some((start, end)) = self.loop_region {
            if position < start as f64 || position >= end as f64 {
                return Err(EngineError::InvalidPosition);
            }
        }
        self.position = position;
        Ok(())
    }

    /// 1.0 = normal, 0.0 = stationary, negative = reverse.
    /// This does not implement pitch-preserving time-stretching.
    pub fn set_playback_rate(&mut self, rate: f64) -> Result<(), EngineError> {
        if !rate.is_finite() || !(-4.0..=4.0).contains(&rate) {
            return Err(EngineError::InvalidPlaybackRate);
        }
        self.playback_rate = rate;
        Ok(())
    }

    pub fn set_gain(&mut self, gain: f32) -> Result<(), EngineError> {
        if !gain.is_finite() || !(0.0..=4.0).contains(&gain) {
            return Err(EngineError::InvalidGain);
        }
        self.gain = gain;
        Ok(())
    }

    pub fn set_crossfader_side(&mut self, side: CrossfaderSide) {
        self.side = side;
    }

    /// Pre-fader cue monitoring independent of the crossfader.
    pub fn set_cue_enabled(&mut self, enabled: bool) {
        self.cue_enabled = enabled;
    }

    /// Enable an exclusive-end [start, end) loop measured in source frames.
    pub fn set_loop(&mut self, start: usize, end: usize) -> Result<(), EngineError> {
        if start >= end || end > self.pcm.len() {
            return Err(EngineError::InvalidLoop);
        }
        self.loop_region = Some((start, end));
        if self.position < start as f64 || self.position >= end as f64 {
            self.position = start as f64;
        }
        Ok(())
    }

    pub fn clear_loop(&mut self) {
        self.loop_region = None;
    }

    fn next_frame(&mut self) -> StereoFrame {
        if !self.playing
            || self.pcm.is_empty()
            || self.position < 0.0
            || self.position >= self.pcm.len() as f64
        {
            if self.playing {
                self.playing = false;
            }
            return StereoFrame::SILENCE;
        }

        let first_index = self.position.floor() as usize;
        let fraction = (self.position - first_index as f64) as f32;
        let second_index = match self.loop_region {
            Some((start, end)) if first_index + 1 >= end => start,
            _ => (first_index + 1).min(self.pcm.len() - 1),
        };

        let first = self.pcm[first_index];
        let second = self.pcm[second_index];
        let output = StereoFrame::new(
            first.left + (second.left - first.left) * fraction,
            first.right + (second.right - first.right) * fraction,
        );

        self.position += self.playback_rate;
        if let Some((start, end)) = self.loop_region {
            let length = (end - start) as f64;
            self.position = start as f64 + (self.position - start as f64).rem_euclid(length);
        }
        output
    }
}

/// Four independently operated decks with stereo master and stereo cue buses.
#[derive(Debug)]
pub struct Engine {
    sample_rate: u32,
    decks: [Deck; 4],
    crossfader: f32,
}

impl Engine {
    pub const DECK_COUNT: usize = 4;

    pub fn new(sample_rate: u32) -> Result<Self, EngineError> {
        if !(8_000..=192_000).contains(&sample_rate) {
            return Err(EngineError::InvalidSampleRate);
        }
        let decks = std::array::from_fn(|index| {
            Deck::new(if index % 2 == 0 {
                CrossfaderSide::Left
            } else {
                CrossfaderSide::Right
            })
        });
        Ok(Self {
            sample_rate,
            decks,
            crossfader: 0.0,
        })
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Zero-based deck indices: 0=A, 1=B, 2=C, 3=D.
    pub fn deck(&self, index: usize) -> Option<&Deck> {
        self.decks.get(index)
    }

    /// Exclusive mutation only; do not call concurrently with render_into.
    pub fn deck_mut(&mut self, index: usize) -> Option<&mut Deck> {
        self.decks.get_mut(index)
    }

    /// -1.0 = left only; +1.0 = right only; 0.0 = equal-power midpoint.
    pub fn set_crossfader(&mut self, position: f32) -> Result<(), EngineError> {
        if !position.is_finite() || !(-1.0..=1.0).contains(&position) {
            return Err(EngineError::InvalidCrossfader);
        }
        self.crossfader = position;
        Ok(())
    }

    /// Fill preallocated interleaved stereo buffers; both must have equal,
    /// even lengths. A safety clamp prevents out-of-range output, but is NOT
    /// a professional-grade limiter.
    pub fn render_into(&mut self, master: &mut [f32], cue: &mut [f32]) -> Result<(), EngineError> {
        if master.len() != cue.len() || !master.len().is_multiple_of(2) {
            return Err(EngineError::InvalidOutputBuffers);
        }

        let left_bus = (0.5 * (1.0 - self.crossfader)).sqrt();
        let right_bus = (0.5 * (1.0 + self.crossfader)).sqrt();

        for (master_frame, cue_frame) in master
            .as_chunks_mut::<2>()
            .0
            .iter_mut()
            .zip(cue.as_chunks_mut::<2>().0.iter_mut())
        {
            let mut master_left = 0.0_f32;
            let mut master_right = 0.0_f32;
            let mut cue_left = 0.0_f32;
            let mut cue_right = 0.0_f32;

            for deck in &mut self.decks {
                let frame = deck.next_frame();
                let fader_gain = match deck.side {
                    CrossfaderSide::Left => left_bus,
                    CrossfaderSide::Right => right_bus,
                    CrossfaderSide::Bypass => 1.0,
                };
                master_left += frame.left * deck.gain * fader_gain;
                master_right += frame.right * deck.gain * fader_gain;
                if deck.cue_enabled {
                    cue_left += frame.left;
                    cue_right += frame.right;
                }
            }

            master_frame[0] = master_left.clamp(-1.0, 1.0);
            master_frame[1] = master_right.clamp(-1.0, 1.0);
            cue_frame[0] = cue_left.clamp(-1.0, 1.0);
            cue_frame[1] = cue_right.clamp(-1.0, 1.0);
        }
        Ok(())
    }
}

/// A pull-style stereo output processor usable by the device adapter.
/// Implementations must not allocate, lock, or perform I/O in the render path.
pub trait StereoRenderer {
    fn render_into(&mut self, master: &mut [f32], cue: &mut [f32]) -> Result<(), EngineError>;
}

impl StereoRenderer for Engine {
    fn render_into(&mut self, master: &mut [f32], cue: &mut [f32]) -> Result<(), EngineError> {
        Engine::render_into(self, master, cue)
    }
}
