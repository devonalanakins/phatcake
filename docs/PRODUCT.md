# Phatcake product requirements

Status: requirements and proposals, not shipped features.

## Four-deck performance (minimum)

Four independently operated decks; load, transport, precise seek, pitch/key, hot cue, beatgrid, loops, waveform, scratching, effects and flexible channel routing. The eventual professional mixer requires gain, EQ, faders, filters, selectable crossfader curves, stereo meters and independent master/pre-cue output. A separate DJ controller is optional, not required.

## Touch and feel

Direct scratch-zone tracking with no forced quantization; user-configurable slip, release, inertia and performance locks; reliable multi-finger input, safe loading, accessible controls and fallback layouts. Graphics must never stall audio.

## Production (after DJ proof)

Live loop capture, sampler, drum pads, clip launcher, MIDI, sequencer, synthesizers, automation, multitrack recording, arrangement, export and third-party plug-in interoperability on a shared session/transport model.

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
5. DAW: music production features added only after credible four-deck performance.
