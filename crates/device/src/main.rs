//! Device-output smoke test, not a touchscreen DJ application.
//! Usage: phatcake-device --probe | --demo SECONDS

use std::error::Error;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, StreamConfig};
use phatcake_control::{control_channel, Command, ControlChange, DeckId};
use phatcake_device::{prepare_demo_engine, OutputScratch};

#[derive(Default)]
struct Diagnostics {
    callbacks: AtomicU64,
    frames: AtomicU64,
    render_errors: AtomicU64,
    stream_errors: AtomicU64,
    xruns: AtomicU64,
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let duration = match args.as_slice() {
        [command] if command == "--probe" => None,
        [command, seconds] if command == "--demo" => {
            let seconds: u64 = seconds.parse()?;
            if !(1..=30).contains(&seconds) {
                return Err("Demo duration must be between 1 and 30 seconds".into());
            }
            Some(Duration::from_secs(seconds))
        }
        _ => {
            println!("Usage: phatcake-device --probe | --demo SECONDS (1-30)");
            return Ok(());
        }
    };

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or("No default audio output device was found")?;
    let supported = device.default_output_config()?;
    let channels = supported.channels() as usize;
    let sample_rate = supported.sample_rate();
    let format = supported.sample_format();

    println!("Default output: {channels} channels, {sample_rate} Hz, {format:?}");
    if duration.is_none() {
        println!("Probe only. No audio stream was started.");
        return Ok(());
    }

    if format != SampleFormat::F32 {
        return Err(
            format!("The first output demo requires f32 audio; the default is {format:?}").into(),
        );
    }
    if channels < 2 {
        return Err("The first output demo requires at least two channels".into());
    }

    let config: StreamConfig = supported.into();
    let engine = prepare_demo_engine(sample_rate)
        .map_err(|error| format!("Cannot prepare synthetic audio: {error:?}"))?;
    let (mut control_sender, mut controlled_engine) = control_channel(engine);

    // Demonstrate a deterministic gain change at a sample boundary, without
    // manipulating shared Engine state from the device callback's caller.
    control_sender
        .send(Command {
            at_frame: sample_rate as u64,
            change: ControlChange::SetGain {
                deck: DeckId::A,
                gain: 0.0,
            },
        })
        .map_err(|error| format!("Cannot schedule mute: {error:?}"))?;
    control_sender
        .send(Command {
            at_frame: (sample_rate as u64) * 2,
            change: ControlChange::SetGain {
                deck: DeckId::A,
                gain: 1.0,
            },
        })
        .map_err(|error| format!("Cannot schedule unmute: {error:?}"))?;
    let mut scratch = OutputScratch::new();

    let diagnostics = Arc::new(Diagnostics::default());
    let callback_metrics = Arc::clone(&diagnostics);
    let error_metrics = Arc::clone(&diagnostics);

    let stream = device.build_output_stream(
        config,
        move |samples: &mut [f32], _: &cpal::OutputCallbackInfo| {
            if scratch.render(&mut controlled_engine, samples, channels).is_err() {
                callback_metrics
                    .render_errors
                    .fetch_add(1, Ordering::Relaxed);
            }
            callback_metrics.callbacks.fetch_add(1, Ordering::Relaxed);
            callback_metrics
                .frames
                .fetch_add((samples.len() / channels) as u64, Ordering::Relaxed);
        },
        move |error| {
            error_metrics.stream_errors.fetch_add(1, Ordering::Relaxed);
            if error.kind() == cpal::ErrorKind::Xrun {
                error_metrics.xruns.fetch_add(1, Ordering::Relaxed);
            }
        },
        None,
    )?;

    println!("Playing four quiet, original synthetic tones with sample-timed deck A gain changes.");
    if channels >= 4 {
        println!("Outputs 1/2: stereo master; 3/4: stereo cue.");
    } else {
        println!("Outputs 1/2: stereo master; cue routing needs a 4+ channel device.");
    }

    stream.play()?;
    std::thread::sleep(duration.expect("valid demo mode"));
    drop(stream);
    drop(control_sender);

    let callbacks = diagnostics.callbacks.load(Ordering::Relaxed);
    let frames = diagnostics.frames.load(Ordering::Relaxed);
    let renders = diagnostics.render_errors.load(Ordering::Relaxed);
    let errors = diagnostics.stream_errors.load(Ordering::Relaxed);
    let xruns = diagnostics.xruns.load(Ordering::Relaxed);
    println!("Callbacks: {callbacks}; rendered frames: {frames}");
    println!("Render errors: {renders}; stream errors: {errors}; reported xruns: {xruns}");
    println!("These counters do NOT measure touch-to-sound or DAC latency.");

    if callbacks == 0 || renders != 0 || errors != 0 {
        return Err("The output demo reported missing callbacks or processing errors".into());
    }

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Phatcake device demo: {error}");
        std::process::exit(1);
    }
}
