# Timestamped audio control — engineering slice

**Status:** Synthetic sample-timing implementation and cross-thread API, not a touchscreen input system or physical latency benchmark.

## Ownership and real-time model

- The non-audio coordination thread owns one `ControlSender`, and the audio callback exclusively owns one `ControlledEngine`.
- A preallocated wait-free ring (`rtrb` 0.4.1, MIT/Apache-2.0) contains up to 1024 small, copy-only control messages. Queue push and pop return immediately; full queue errors never overwrite pending events.
- Every message has an **absolute output-rendering frame number**, not a wall-clock or DAC playback timestamp.
- Commands may be queued only in nondecreasing frame order. Equal-frame commands retain FIFO order; out-of-order calls are rejected. A future timestamp already queued can block an urgent command in this first *single-lane* prototype. A priority lane for immediate scratch gestures must be designed before claiming professional responsiveness.
- Rendering polls the command queue at most every 32 output frames and splits work at known future command boundaries. Events already available before their frame are applied before rendering that exact frame.
- There is a finite 64-event budget per render call. Heavy event floods may delay controls; deferred and late metrics make this visible.
- The audio thread never blocks on the UI or graphics thread. Audio buffers are owned by the callback and generated without per-callback allocation or locking in Phatcake's source.
- The atomic published clock allows the producer to read the latest rendered frame count. It is **not** an audio hardware timebase, so its use alone cannot establish physical input-to-sound latency.

## Supported changes

Play, pause, stop, seek, playback rate (including reverse), per-deck gain, per-deck cue enable, loop set/clear and master crossfader. A typed deck enum (A/B/C/D) prevents invalid deck numbers. The DSP engine rejects invalid numerical arguments without changing state.

## Verification

Deterministic tests cover exact-frame commands across buffer boundaries, FIFO equal-frame application, pause/resume, out-of-order and full-queue behavior, invalid values, late arrival, cross-thread producer transfer and invalid output buffers. A device-integration test verifies that the native stereo adapter accepts the controlled renderer.

The native `--demo` runs four copyright-free synthetic tones and schedules a deck A gain mute at one second, restoring gain at two seconds. A real audio interface and physical listening equipment are required to verify actual playback.

## Remaining before touchscreen scratching

1. A separate priority path or validated scheduling policy for immediate gestures when future automation is queued.
2. External timebase calibration between touchscreen events, rendered frames, audio driver buffering and audible output.
3. Per-event latency distribution measurement under load, with clear p50/p95/p99 reporting and device configuration.
4. High-quality scratch playback algorithm, interpolators, declicking and controller-free touch interaction.
5. Hardware-directed xrun/dropout endurance tests and professional user evaluation.

No license decision or production release is implied by this experiment.
