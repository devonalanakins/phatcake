# Native audio output proof

Status: experimental native-output candidate. CI success does not establish actual sound output, device compatibility or end-to-end latency.

The independent Squelch offline DSP engine remains dependency-free. A separate squelch-device crate uses pinned RustAudio CPAL 0.18.2 (Apache-2.0) to connect to the operating system's default output device.

## Behavior

- Four quiet, copyright-free sine waves are prepared, loaded, looped and started before the sound stream begins.
- The audio callback exclusively owns the engine and two fixed-size scratch buffers. It does not allocate, lock, log, access disk or perform network I/O.
- The first two device channels carry stereo master. If the default device exposes at least four output channels, channels 3/4 carry independent pre-fader cue; additional channels carry silence.
- The command fails clearly if no output device exists, fewer than two channels are exposed, or the default output sample format is not f32.
- Atomics count callback invocations, processed frames and reported errors. These are diagnostic counts only, not latency measurements or verified no-dropout results.

## Commands

Run with stable Rust. Linux requires ALSA development headers (libasound2-dev).

    cargo run -p squelch-device -- --probe
    cargo run -p squelch-device -- --demo 5
    cargo test --workspace --all-targets

Probe inspects the device without producing sound. Demo runs for 1 to 30 seconds and emits synthetic tones, not commercial music. Set speaker/headphone volume low before starting.

## Not supported in this candidate

- Custom output device selection; integer or nondefault sample formats; a chosen four-channel configuration when the device defaults to stereo.
- Music file loading, stems, scratching, multitouch, independent USB headphone device routing, DAW features, recording, plug-ins or persistence.
- Time-stamped cross-thread control delivery, qualified driver/OS optimizations, physically measured latency, xruns from all device drivers, or live show reliability.
- Automatic verification of audible sound in CI. Unit tests validate only deterministic, synthetic buffer routing.

Next gate: qualified hardware output smoke tests, a bounded sample-time command queue, dropout instrumentation, and independent touch-to-sound measurement.

PR integration remains subject to review; no merge, deployment or product release is implied.
