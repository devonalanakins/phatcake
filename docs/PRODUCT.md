# Squelch product requirements

Status: requirements and proposals, not shipped features.

## Four-deck performance (minimum)

Four independently operated decks; load, transport, precise seek, pitch/key, hot cue, beatgrid, loops, waveform, scratching, effects and flexible channel routing. The eventual professional mixer requires gain, EQ, faders, filters, selectable crossfader curves, stereo meters and independent master/pre-cue output. A separate DJ controller is optional, not required.

## Touch and feel

Direct scratch-zone tracking with no forced quantization; user-configurable slip, release, inertia and performance locks; reliable multi-finger input, safe loading, accessible controls and fallback layouts. Graphics must never stall audio.

## Unified DAW and performance product charter (locked vision; staged implementation)

Squelch is not a standalone DJ app with a loosely attached future DAW. The long-term goal is one complete professional environment for DJ performance, sound design, instruments, multitrack recording, composition, mixing, mastering and live performance. All domains must share a future canonical audio/session/automation graph and support performance-to-arrangement and arrangement-to-deck workflows.

**Implementation order is still gated by validated four-deck DJ audio.** Designing the unified DAW foundation now avoids a later rewrite, but does not imply production features already exist. Complete product scope, physical hardware limits, experience principles, acceptance measures and staged delivery are defined in [Squelch unified vision](VISION.md).

Live loop capture, sampler, drum pads, clip launcher, MIDI, sequencer, synthesizers, automation, multitrack recording, arrangement, export and third-party plug-in interoperability remain prospective requirements.

## Measurable engineering targets — not achieved

- Four-deck stable playback on a specified, qualified reference machine and wired audio device.
- Candidate p95 touch-to-sound response <=20 ms, stretch <10 ms when technically demonstrable; requires external end-to-end measurement, not audio-buffer arithmetic.
- Evaluate 64/128/256-frame buffers at 48 kHz on supported drivers; never advertise a universal guarantee.
- 120 fps graphics where supported, independent of DSP and input timing.
- Two-hour synthetic four-deck test with zero audible dropouts as a target, followed by field tests with working DJs.

## Gates

1. Offline DSP correctness: four channels, independent transports, loops, routing and cue.
2. Native audio: device I/O, dropout diagnostics and physical latency measurements.
3. Scratch UX: direct-touch precision, safe multitouch, comparison with DVS/controller benchmarks.
4. Professional live workflows: track library, recovery, session recording, effects and usability validation.
5. DAW: design a shared session/transport/audio graph now; implement and release recording, composition, instrument, mixing, mastering and plugin modules incrementally after credible DJ performance milestones.
