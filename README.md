# Phatcake

**Touch the sound.**

Phatcake is a developing, touchscreen-first, professional music-performance and music-production platform. Its core promise is four simultaneous digital DJ decks without required physical turntables or MIDI controllers, with a future unified DAW.

**Status: experimental offline DSP source only. This is NOT a playable DJ application.** There is no audio-device I/O, touchscreen interface, file decoder, real-time event transport, pitch-preserving stretching, or production DAW yet.

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
