# Phatcake

**Touch the sound.**

Phatcake is a developing, touchscreen-first, professional music-performance and music-production platform. Its core promise is four simultaneous digital DJ decks without required physical turntables or MIDI controllers, with a future unified DAW.

**Status: experimental engineering previews only, NOT a playable touchscreen DJ application.** There is no audio-device I/O, touchscreen interface, file decoder, real-time event transport, pitch-preserving stretching, or production DAW yet.

## Established requirements

- **Four decks minimum:** independently playable, controlled, scratched, cued and mixed.
- **Touch-native:** precise gestures, multi-finger operation, and no mandatory DJ controller.
- **Real-time reliability:** separate audio scheduling from visualization, operate offline, and measure latency on actual hardware.
- **Cue/master independence:** professional headphone cueing with a suitable audio-output interface.
- **Long-term DJ + DAW:** record, remix, sequence and arrange within the same audio foundation after DJ feasibility is established.

## Implemented first slice

The dependency-free Rust crate `crates/engine` can render four independent pre-decoded stereo PCM decks to offline stereo master/cue buffers. It implements play, pause, stop, seek, forward/reverse/fractional rates, loop regions, gain, equal-power crossfader buses, and pre-fader cue. The rendering method writes into caller-supplied memory without allocating, blocking, or performing I/O.

Run with the current stable Rust toolchain:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
```

See [architecture](docs/ARCHITECTURE.md), [product requirements](docs/PRODUCT.md), [brand direction](docs/BRAND.md), and [contributing](CONTRIBUTING.md).

## Licensing

**License selection is pending owner approval.** The repository is public, but its content is not yet offered under an OSI-approved license. External contributions and redistribution should wait until a suitable license is selected.

## Native-output smoke test (stacked draft milestone)

The experimental device crate can route the four-deck synthetic master mix to the computer's default stereo output, with headphone cue on channels 3/4 when that device's default configuration has at least four channels. It is not a DJ app or measured low-latency system.

Commands:

    cargo run -p phatcake-device -- --probe
    cargo run -p phatcake-device -- --demo 5

The first output adapter supports f32 default device configurations only. See docs/NATIVE_AUDIO.md for safety, tests and limitations. This milestone is based on the still-unmerged four-deck engine PR.
