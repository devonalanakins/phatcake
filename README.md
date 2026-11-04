# Squelch

**Touch the sound.**

Squelch is a developing, touchscreen-first, professional music-performance and music-production platform. Its ambition is a unified professional DAW and live-performance environment in which four or more digital decks, instruments, recording, composition, mixing, mastering and stage control share one session. Dedicated DJ controllers and other creative control surfaces are optional, but physical audio input/output hardware remains necessary for acoustic capture and reproduction.

**Status: experimental engineering previews only, NOT a playable touchscreen DJ application.** A native f32 output proof and bounded sample-time control scheduler exist on stacked draft branches. There is no touchscreen interface, file decoder, measured latency, professional scratching, or production DAW yet.

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

See [unified product vision](docs/VISION.md), [architecture](docs/ARCHITECTURE.md), [product requirements](docs/PRODUCT.md), [brand direction](docs/BRAND.md), [touch cockpit design](docs/INTERFACE.md), and [contributing](CONTRIBUTING.md).

## Licensing

**License selection is pending owner approval.** The repository is public, but its content is not yet offered under an OSI-approved license. External contributions and redistribution should wait until a suitable license is selected.

## Native-output smoke test (stacked draft milestone)

The experimental device crate can route the four-deck synthetic master mix to the computer's default stereo output, with headphone cue on channels 3/4 when that device's default configuration has at least four channels. It is not a DJ app or measured low-latency system.

Commands:

    cargo run -p squelch-device -- --probe
    cargo run -p squelch-device -- --demo 5

The first output adapter supports f32 default device configurations only. See docs/NATIVE_AUDIO.md for safety, tests and limitations. This milestone is based on the still-unmerged four-deck engine PR.


## Sample-time control scheduling (third stacked draft)

The `squelch-control` crate adds typed per-deck controls over a fixed-capacity wait-free SPSC queue, timestamped in absolute audio-rendered frames. Already-enqueued events can take effect on exact sample boundaries. Invalid controls are rejected and counted, full queues apply backpressure, and late commands are counted rather than silently discarded. `squelch-device` now accepts either the original engine or its controlled wrapper without any render-path heap allocations.

The native synthetic demo has a scheduled gain change at one second and restoration at two seconds, with no touchscreen UI. It is *not* a measured low-latency scratch implementation.

See [control scheduling](docs/CONTROL.md) and [native output](docs/NATIVE_AUDIO.md). This PR is stacked on the still-unmerged native-output proof.
