# Architecture: the offline four-deck foundation

## Thread and component boundaries

- **Touch interface (future):** native multitouch controls and a GPU UI independent of audio scheduling.
- **Control/event transport (future):** timestamped, bounded, real-time-safe delivery of gestures and control changes to the audio owner.
- **Audio engine (partial):** `squelch-engine` renders four independent stereo PCM sources to separate master and headphone cue outputs.
- **Background workers (future):** file decoding, sample-rate conversion, beatgrids, stems, waveform preparation, library indexing, persistence and recovery.
- **Native audio output (future):** device routing, hardware measurement, driver compatibility and recovery.

```text
[touch input] --> [future command queue] --> [single-owner engine: A B C D]
                                                    |          |
                                                master       cue
                                                    |          |
                                                [future device backend]

[file decode / analysis] --> [future non-RT preparation] --> [engine]
```

## Explicit limitations

- All supplied buffers are already decoded, finite stereo PCM at the engine's rate. Track loading is NOT part of real-time rendering.
- The caller must own the engine exclusively; there is no concurrency-safe UI-to-audio command queue yet.
- Fractional speed uses linear sample interpolation, not professional pitch-preserving time-stretching, anti-aliasing, or DVS-quality scratching.
- Loops use half-open source-frame ranges, forward and reverse wrapped.
- Crossfader is equal power with A/C initially left, B/D initially right; any deck can bypass.
- Cue is pre-channel-gain and pre-crossfader, with monitoring gain controls deferred.
- A hard numerical clamp is a provisional safety measure, not a transparent limiter. Further mixer architecture is required to avoid audible clipping.
- The first engine cannot substantiate any claims of low latency, zero dropouts, live readiness or superiority to Traktor.

## Next bounded engineering slice

1. Wire an appropriate native audio-output backend for a reference device.
2. Introduce deterministic timestamped control events without taking locks on the audio thread.
3. Design external physical touch-to-sound latency measurement and a two-hour four-deck stability test.
4. Prototype touch scratching after those contracts are testable.

## Ownership boundaries

Squelch is an independent music-software project; do not share private data, billing, deployments or secrets from DevonStation, CristyBreathe, Brandicade or Anevum.
