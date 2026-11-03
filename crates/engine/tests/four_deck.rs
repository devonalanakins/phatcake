use phatcake_engine::{CrossfaderSide, Engine, EngineError, StereoFrame};

fn pcm(values: &[f32]) -> Vec<StereoFrame> {
    values
        .iter()
        .copied()
        .map(|value| StereoFrame::new(value, value))
        .collect()
}

fn near(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < 0.00001,
        "expected {expected}, got {actual}"
    );
}

#[test]
fn mixes_four_simultaneous_decks() {
    let mut engine = Engine::new(48_000).unwrap();
    for (index, level) in [0.1, 0.2, 0.3, 0.4].iter().copied().enumerate() {
        let deck = engine.deck_mut(index).unwrap();
        deck.load_pcm(pcm(&[level; 8])).unwrap();
        deck.play().unwrap();
    }
    let mut master = [0.0; 8];
    let mut cue = [0.0; 8];
    engine.render_into(&mut master, &mut cue).unwrap();
    let expected = 1.0 * std::f32::consts::FRAC_1_SQRT_2;
    for frame in master.as_chunks::<2>().0.iter() {
        near(frame[0], expected);
        near(frame[1], expected);
    }
    assert!(cue.iter().all(|sample| *sample == 0.0));
}

#[test]
fn crossfader_routes_left_and_right_independently() {
    let mut engine = Engine::new(48_000).unwrap();
    for (index, level) in [0.25, 0.75].iter().copied().enumerate() {
        let deck = engine.deck_mut(index).unwrap();
        deck.load_pcm(pcm(&[level; 8])).unwrap();
        deck.play().unwrap();
    }
    let mut master = [0.0; 2];
    let mut cue = [0.0; 2];
    engine.set_crossfader(-1.0).unwrap();
    engine.render_into(&mut master, &mut cue).unwrap();
    near(master[0], 0.25);
    engine.set_crossfader(1.0).unwrap();
    engine.render_into(&mut master, &mut cue).unwrap();
    near(master[0], 0.75);
}

#[test]
fn headphone_cue_remains_prefader() {
    let mut engine = Engine::new(48_000).unwrap();
    let deck = engine.deck_mut(1).unwrap();
    deck.load_pcm(pcm(&[0.5; 4])).unwrap();
    deck.set_gain(0.0).unwrap();
    deck.set_cue_enabled(true);
    deck.play().unwrap();
    engine.set_crossfader(-1.0).unwrap();
    let mut master = [0.0; 4];
    let mut cue = [0.0; 4];
    engine.render_into(&mut master, &mut cue).unwrap();
    assert!(master.iter().all(|value| *value == 0.0));
    assert!(cue.iter().all(|value| *value == 0.5));
}

#[test]
fn loops_wrap_and_remain_bounded() {
    let mut engine = Engine::new(48_000).unwrap();
    let deck = engine.deck_mut(0).unwrap();
    deck.load_pcm(pcm(&[0.1, 0.2, 0.3, 0.4])).unwrap();
    deck.set_loop(1, 3).unwrap();
    deck.play().unwrap();
    engine.set_crossfader(-1.0).unwrap();
    let mut master = [0.0; 8];
    let mut cue = [0.0; 8];
    engine.render_into(&mut master, &mut cue).unwrap();
    for (frame, expected) in master.as_chunks::<2>().0.iter().zip([0.2, 0.3, 0.2, 0.3]) {
        near(frame[0], expected);
    }
}

#[test]
fn reverse_and_fractional_scrubbing_work_offline() {
    let mut engine = Engine::new(48_000).unwrap();
    let deck = engine.deck_mut(0).unwrap();
    deck.load_pcm(pcm(&[0.1, 0.2, 0.3, 0.4])).unwrap();
    deck.seek(2.0).unwrap();
    deck.set_playback_rate(-1.0).unwrap();
    deck.play().unwrap();
    engine.set_crossfader(-1.0).unwrap();
    let mut master = [0.0; 8];
    let mut cue = [0.0; 8];
    engine.render_into(&mut master, &mut cue).unwrap();
    for (frame, expected) in master.as_chunks::<2>().0.iter().zip([0.3, 0.2, 0.1, 0.0]) {
        near(frame[0], expected);
    }
    let deck = engine.deck_mut(0).unwrap();
    deck.seek(1.5).unwrap();
    deck.set_playback_rate(0.0).unwrap();
    deck.play().unwrap();
    engine.render_into(&mut master[..2], &mut cue[..2]).unwrap();
    near(master[0], 0.25);
}

#[test]
fn invalid_controls_and_output_buffers_are_rejected() {
    let mut engine = Engine::new(48_000).unwrap();
    assert!(engine.deck(4).is_none());
    assert!(engine.deck_mut(4).is_none());
    assert!(matches!(
        Engine::new(0),
        Err(EngineError::InvalidSampleRate)
    ));
    assert_eq!(
        engine.deck_mut(0).unwrap().play(),
        Err(EngineError::NoTrackLoaded)
    );
    let deck = engine.deck_mut(0).unwrap();
    assert_eq!(deck.load_pcm(pcm(&[])), Err(EngineError::InvalidTrack));
    deck.load_pcm(pcm(&[0.25; 4])).unwrap();
    assert_eq!(deck.set_gain(f32::NAN), Err(EngineError::InvalidGain));
    assert_eq!(
        deck.set_playback_rate(f64::INFINITY),
        Err(EngineError::InvalidPlaybackRate)
    );
    assert_eq!(deck.set_loop(3, 5), Err(EngineError::InvalidLoop));
    assert_eq!(deck.seek(9.0), Err(EngineError::InvalidPosition));
    deck.set_crossfader_side(CrossfaderSide::Bypass);
    deck.play().unwrap();
    engine.set_crossfader(1.0).unwrap();
    let mut master = [0.0; 2];
    let mut cue = [0.0; 2];
    engine.render_into(&mut master, &mut cue).unwrap();
    near(master[0], 0.25);
    let mut bad = [9.0; 3];
    assert_eq!(
        engine.render_into(&mut bad, &mut cue),
        Err(EngineError::InvalidOutputBuffers)
    );
    assert_eq!(bad, [9.0; 3]);
}
