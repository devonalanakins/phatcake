use phatcake_control::{
    control_channel, Command, ControlChange, DeckId, SendError, COMMAND_CAPACITY,
};
use phatcake_engine::{Engine, StereoFrame};

fn engine() -> Engine {
    let mut engine = Engine::new(48_000).unwrap();
    engine.set_crossfader(-1.0).unwrap();
    let deck = engine.deck_mut(0).unwrap();
    deck.load_pcm(vec![StereoFrame::new(0.25, 0.25); 2048])
        .unwrap();
    deck.play().unwrap();
    engine
}

fn render_samples(player: &mut phatcake_control::ControlledEngine, frames: usize) -> Vec<f32> {
    let mut master = vec![0.0_f32; frames * 2];
    let mut cue = vec![0.0_f32; frames * 2];
    player.render_into(&mut master, &mut cue).unwrap();
    master
        .as_chunks::<2>()
        .0
        .iter()
        .map(|frame| frame[0])
        .collect()
}

fn close(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < 0.00001,
        "expected {expected}, got {actual}"
    );
}

#[test]
fn applies_gain_on_the_exact_sample_across_callback_boundaries() {
    let (mut sender, mut player) = control_channel(engine());
    sender
        .send(Command {
            at_frame: 5,
            change: ControlChange::SetGain {
                deck: DeckId::A,
                gain: 0.0,
            },
        })
        .unwrap();
    let before = render_samples(&mut player, 3);
    let after = render_samples(&mut player, 6);
    for sample in before.iter().chain(after[..2].iter()) {
        close(*sample, 0.25);
    }
    for sample in &after[2..] {
        close(*sample, 0.0);
    }
    assert_eq!(player.current_frame(), 9);
    assert_eq!(sender.current_frame(), 9);
    assert_eq!(player.stats().applied, 1);
    assert_eq!(player.stats().late, 0);
}

#[test]
fn equal_frame_commands_execute_in_fifo_order() {
    let (mut sender, mut player) = control_channel(engine());
    for value in [0.0, 0.5] {
        sender
            .send(Command {
                at_frame: 2,
                change: ControlChange::SetGain {
                    deck: DeckId::A,
                    gain: value,
                },
            })
            .unwrap();
    }
    let audio = render_samples(&mut player, 5);
    for sample in &audio[..2] {
        close(*sample, 0.25);
    }
    for sample in &audio[2..] {
        close(*sample, 0.125);
    }
    assert_eq!(player.stats().applied, 2);
}

#[test]
fn pause_and_resume_are_sample_accurate() {
    let (mut sender, mut player) = control_channel(engine());
    sender
        .send(Command {
            at_frame: 2,
            change: ControlChange::Pause(DeckId::A),
        })
        .unwrap();
    sender
        .send(Command {
            at_frame: 4,
            change: ControlChange::Play(DeckId::A),
        })
        .unwrap();
    let audio = render_samples(&mut player, 6);
    for (actual, expected) in audio.iter().zip([0.25, 0.25, 0.0, 0.0, 0.25, 0.25]) {
        close(*actual, expected);
    }
}

#[test]
fn sender_rejects_out_of_order_commands_without_reordering() {
    let (mut sender, mut player) = control_channel(engine());
    sender
        .send(Command {
            at_frame: 10,
            change: ControlChange::SetCrossfader(-1.0),
        })
        .unwrap();
    assert_eq!(
        sender.send(Command {
            at_frame: 9,
            change: ControlChange::SetCrossfader(1.0),
        }),
        Err(SendError::OutOfOrder {
            previous: 10,
            requested: 9,
        })
    );
    let audio = render_samples(&mut player, 12);
    assert!(audio.iter().all(|sample| (*sample - 0.25).abs() < 0.00001));
    assert_eq!(player.stats().applied, 1);
}

#[test]
fn full_queue_never_discards_or_overwrites_existing_events() {
    let (mut sender, mut player) = control_channel(engine());
    for _ in 0..COMMAND_CAPACITY {
        sender
            .send(Command {
                at_frame: 0,
                change: ControlChange::SetGain {
                    deck: DeckId::A,
                    gain: 0.5,
                },
            })
            .unwrap();
    }
    assert!(matches!(
        sender.send(Command {
            at_frame: 0,
            change: ControlChange::Stop(DeckId::A),
        }),
        Err(SendError::Full(_))
    ));
    let audio = render_samples(&mut player, 1);
    close(audio[0], 0.125);
    assert_eq!(player.stats().applied, 64);
    assert!(player.stats().deferred > 0);
    // Each further render invocation drains at most 64 events.
    for _ in 0..15 {
        render_samples(&mut player, 1);
    }
    assert_eq!(player.stats().applied, COMMAND_CAPACITY as u64);
    assert!(player.deck(DeckId::A).is_playing());
}

#[test]
fn invalid_commands_are_rejected_and_do_not_change_audio() {
    let (mut sender, mut player) = control_channel(engine());
    sender
        .send(Command {
            at_frame: 0,
            change: ControlChange::SetGain {
                deck: DeckId::A,
                gain: f32::NAN,
            },
        })
        .unwrap();
    let audio = render_samples(&mut player, 3);
    assert!(audio.iter().all(|sample| (*sample - 0.25).abs() < 0.00001));
    assert_eq!(player.stats().invalid, 1);
    assert_eq!(player.stats().applied, 0);
}

#[test]
fn late_commands_are_counted_but_applied_on_next_poll() {
    let (mut sender, mut player) = control_channel(engine());
    render_samples(&mut player, 10);
    sender
        .send(Command {
            at_frame: 3,
            change: ControlChange::SetGain {
                deck: DeckId::A,
                gain: 0.0,
            },
        })
        .unwrap();
    let audio = render_samples(&mut player, 4);
    assert!(audio.iter().all(|sample| *sample == 0.0));
    assert_eq!(player.stats().late, 1);
    assert_eq!(player.stats().applied, 1);
}

#[test]
fn commands_can_cross_threads_without_accessing_engine_mutably() {
    let (sender, mut player) = control_channel(engine());
    let producer = std::thread::spawn(move || {
        let mut sender = sender;
        sender
            .send(Command {
                at_frame: 4,
                change: ControlChange::SetGain {
                    deck: DeckId::A,
                    gain: 0.0,
                },
            })
            .unwrap();
    });
    producer.join().unwrap();
    let audio = render_samples(&mut player, 6);
    for sample in &audio[..4] {
        close(*sample, 0.25);
    }
    for sample in &audio[4..] {
        close(*sample, 0.0);
    }
}

#[test]
fn invalid_output_buffers_do_not_consume_commands() {
    let (mut sender, mut player) = control_channel(engine());
    sender
        .send(Command {
            at_frame: 0,
            change: ControlChange::SetGain {
                deck: DeckId::A,
                gain: 0.0,
            },
        })
        .unwrap();
    let mut master = [7.0; 3];
    let mut cue = [8.0; 2];
    assert_eq!(
        player.render_into(&mut master, &mut cue),
        Err(phatcake_engine::EngineError::InvalidOutputBuffers)
    );
    assert_eq!(master, [7.0; 3]);
    assert_eq!(cue, [8.0; 2]);
    assert_eq!(player.current_frame(), 0);
    assert_eq!(player.stats().applied, 0);
    assert!(render_samples(&mut player, 1)[0] == 0.0);
}
