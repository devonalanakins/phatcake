use phatcake_control::{control_channel, Command, ControlChange, DeckId};
use phatcake_device::{prepare_demo_engine, OutputError, OutputScratch};
use phatcake_engine::{Engine, StereoFrame};

fn close(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < 0.00001,
        "expected {expected}, got {actual}"
    );
}

fn constant_engine(value: f32, cue_enabled: bool) -> Engine {
    let mut engine = Engine::new(48_000).unwrap();
    let deck = engine.deck_mut(0).unwrap();
    deck.load_pcm(vec![StereoFrame::new(value, -value); 500])
        .unwrap();
    deck.set_cue_enabled(cue_enabled);
    deck.play().unwrap();
    engine.set_crossfader(-1.0).unwrap();
    engine
}

#[test]
fn stereo_output_routes_only_master() {
    let mut engine = constant_engine(0.25, true);
    let mut scratch = OutputScratch::new();
    let mut data = [9.0; 6];
    scratch.render(&mut engine, &mut data, 2).unwrap();
    for frame in data.as_chunks::<2>().0 {
        close(frame[0], 0.25);
        close(frame[1], -0.25);
    }
}

#[test]
fn four_channel_device_routes_independent_cue() {
    let mut engine = constant_engine(0.25, true);
    engine.deck_mut(0).unwrap().set_gain(0.5).unwrap();
    let mut scratch = OutputScratch::new();
    let mut data = [9.0; 8];
    scratch.render(&mut engine, &mut data, 4).unwrap();
    for frame in data.as_chunks::<4>().0 {
        close(frame[0], 0.125);
        close(frame[1], -0.125);
        close(frame[2], 0.25);
        close(frame[3], -0.25);
    }
}

#[test]
fn extra_device_channels_are_silent() {
    let mut engine = constant_engine(0.25, true);
    let mut scratch = OutputScratch::new();
    let mut data = [9.0; 12];
    scratch.render(&mut engine, &mut data, 6).unwrap();
    for frame in data.as_chunks::<6>().0 {
        close(frame[0], 0.25);
        close(frame[1], -0.25);
        close(frame[2], 0.25);
        close(frame[3], -0.25);
        close(frame[4], 0.0);
        close(frame[5], 0.0);
    }
}

#[test]
fn invalid_layout_is_silent_and_preserves_transport() {
    let mut engine = constant_engine(0.25, true);
    let mut scratch = OutputScratch::new();
    let mut data = [9.0; 5];
    let position = engine.deck(0).unwrap().position_frames();
    assert_eq!(
        scratch.render(&mut engine, &mut data, 2),
        Err(OutputError::InvalidLayout)
    );
    assert_eq!(data, [0.0; 5]);
    assert_eq!(engine.deck(0).unwrap().position_frames(), position);
    assert_eq!(
        scratch.render(&mut engine, &mut data, 1),
        Err(OutputError::InvalidLayout)
    );
}

#[test]
fn handles_callback_buffers_larger_than_one_block() {
    let mut engine = constant_engine(0.25, false);
    let mut scratch = OutputScratch::new();
    let mut data = [0.0; 514];
    scratch.render(&mut engine, &mut data, 2).unwrap();
    close(data[0], 0.25);
    close(data[512], 0.25);
    close(data[513], -0.25);
    close(engine.deck(0).unwrap().position_frames() as f32, 257.0);
}

#[test]
fn prepared_four_deck_mix_is_non_silent() {
    let mut engine = prepare_demo_engine(48_000).unwrap();
    assert_eq!(Engine::DECK_COUNT, 4);
    for index in 0..4 {
        assert!(engine.deck(index).unwrap().is_playing());
    }
    let mut scratch = OutputScratch::new();
    let mut data = [0.0; 512];
    scratch.render(&mut engine, &mut data, 2).unwrap();
    assert!(data.iter().all(|sample| sample.is_finite()));
    assert!(data.iter().any(|sample| sample.abs() > 0.01));
}

#[test]
fn invalid_sample_rate_does_not_allocate_test_sources() {
    assert!(prepare_demo_engine(0).is_err());
}


#[test]
fn device_adapter_applies_timed_control_to_interleaved_audio() {
    let engine = constant_engine(0.25, true);
    let (mut sender, mut controlled) = control_channel(engine);
    sender
        .send(Command {
            at_frame: 3,
            change: ControlChange::SetGain {
                deck: DeckId::A,
                gain: 0.0,
            },
        })
        .unwrap();

    let mut scratch = OutputScratch::new();
    let mut data = [0.0; 20];
    scratch.render(&mut controlled, &mut data, 2).unwrap();

    for (frame_index, frame) in data.as_chunks::<2>().0.iter().enumerate() {
        let expected = if frame_index < 3 { 0.25 } else { 0.0 };
        close(frame[0], expected);
        close(frame[1], -expected);
    }
    assert_eq!(controlled.stats().applied, 1);
    assert_eq!(controlled.stats().late, 0);
}
