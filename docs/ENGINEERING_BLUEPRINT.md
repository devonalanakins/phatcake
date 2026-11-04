# Squelch — full engineering blueprint

**Status:** Architecture proposal, not implemented or approved final technical stack. Current Rust progress is a four-deck offline PCM engine, experimental native output adapter, and one bounded sample-time control queue on stacked unmerged pull requests. The complete DAW, touchscreen integration, project schema, plugin host and hardware-validation pipeline have NOT been built.

## One canonical session: system design

Squelch is a single musical workspace, not a DJ tool attached to a separate DAW. Perform, Produce, Shape, Mix and Master are views of the same versioned Session, transport clock, graph, media, automation, recording state and undo/recovery history.

The most valuable differentiator is an **editable performance lifecycle**. Load and scratch four independent deck sources; simultaneously capture audio buses and typed touch/mixer events; transform them into clips and arrangement automation; edit those events; reproduce a modified loop back on a deck; then mix and master within the same saved session. This is a design hypothesis that must be demonstrated against existing integrated DAW and DJ workflows rather than advertised as uniquely invented.

### Logical entities

| Entity | Data and invariants |
| --- | --- |
| Session | Stable ID, schema version, revision, sample-rate policy, transport, tempo map, undo/journal |
| Asset | Content hash, relative path, channels, source sample rate, format, rights/user notes, original provenance |
| Deck | A/B/C/D+ ID, asset or live-source reference, playback rate, direction, loop, cue/master buses |
| Track | Audio/MIDI/instrument/bus kind, clips, takes, arm/monitor state, channel routing |
| Clip | Stable ID, source, region, session position, fades, warp, non-destructive lineage |
| GestureTake | Control target, input device, timestamped action samples, engine version and conversion map |
| GraphNode | Typed audio/MIDI/control ports, channel layout, latency, processing capability |
| GraphEdge | Source/destination, channel mapping, gain, validation, feedback-delay requirement |
| Automation | Parameter target, musical/sample timebase, interpolation, event sequence, author |
| Scene | Clip/deck arrangement, scoped recall policy, quantization and safe transition |
| RenderJob | Session revision, source range, output routing, format, rate, metadata, validation/report |

**Timebase:** Asset frames and transport musical time are distinct. Define rational time signature/BPM segments, explicit conversions to signed 64-bit frame units, deterministic rounding and overflow validation. The current rendered-frame counter does NOT describe physical audio DAC time. Never infer measured latency from UI repaint intervals.

**Persistence:** Design a self-describing versioned local project with manifest, operation journal, asset references and optional packed media. A future filename extension is not locked. Atomic writes, schema migrations, hash verification, corruption diagnosis, missing-media relinking, plugin-missing placeholders and recovery checkpoints are required before accepting irreplaceable user recordings.

**Shared object identity:** A clip on a deck and the same clip in arrangement reference a common source lineage. A view transition cannot copy media by default, reset transport or secretly change output routing. UI state, document state and audio engine snapshots need separate revisions.

## Runtime architecture and thread ownership

~~~
TOUCH / KEYS / OPTIONAL MIDI CONTROLLERS
                |
        Typed intent validator
                |
    Session editor + command router ------------> Project journal
         |                 |                             |
         |                 +----> Background media/analysis/exports
         |
         +----> Safety control lane
         +----> Urgent performance lane
         +----> Scheduled automation lane
                          |
                 REALTIME AUDIO OWNER
        Four decks / tracks / instrument nodes
              Mixer + effects + output graph
                          |
                  NATIVE AUDIO BACKEND
                  MASTER + independent CUE
~~~

The above is a future decomposition; current source does not implement the unified graph. Rust remains the preferred DSP/session language. Keep the host window and accessibility UI independent from the audio callback. A Tauri-class HTML/TypeScript UI shell is a candidate because of local-first packaging and semantic controls, but its touch-to-native timing must pass real-device measurements before the UI framework is chosen. Alternative native input bridges are permitted if measurements demand them. Do not commit to cross-platform equality without a driver/OS matrix.

**Graph build:** validate topology and channel assignments, reject illegal feedback without declared delay, prepare nodes/resources off the callback, publish at an explicit boundary, retire old graph state after safe acknowledgement. Plug-in delay compensation, sample-rate changes, offline render support and plugin tail handling must be explicit; live output must never perform disk I/O or dynamic plugin scanning.

**Realtime safety:** callback does no unbounded allocation, locks, file access, network, user input parsing, expensive analysis or unpredictable graph mutation. A UI freeze does not stop audio on purpose. CPU overrun, lost device and unavailable cue buses generate real status events with safe remediation, not fabricated activity meters.

## Priority touch/control contract

The present single FIFO scheduler can let future scheduled commands obstruct emergency or immediate performer gestures. Preserve it as an existing foundation, then build three bounded policies:
1. **Safety lane:** panic/mute, stop, device-loss/recovery. Reserve capacity, bounded processing and explicit acknowledgement; software cannot guarantee safety if the OS/hardware itself fails.
2. **Urgent lane:** scratch start/drag/release, active cue, faders, mixer changes, live instruments. Preserve order per target, coalesce gesture motion only according to documented semantics and never silently discard a required scratch release.
3. **Scheduled lane:** future frame automation, scene triggers and beat-time events. Apply with sample/frame timing when available, never starving urgent lane.

Every command contains target ID, session revision, origin/input ID, event sequence, requested frame/musical time, typed value and validity bounds. Return rejected/queued/applied state separately from hardware-presented audio. Queue overflow and late events must be observable. The UI must not flash "playback successful" solely because a button animation ran.

## Audio infrastructure and physical boundary

Support qualified desktop Windows/macOS/Linux audio backends via a tested native device adapter. A standalone computer with a stereo headphone jack does not automatically provide **independent stereo master and stereo headphone cue**. A four-channel-capable device or a well-documented compromised split mode may be needed. A physical computer, converter, speakers/headphones and microphone/ADC for external recordings cannot be replaced by UI code. Physical motorized platter haptics and dedicated emergency controls can be approximated but not identically reproduced in software.

Design a device-check page: list actual output endpoints, sample rate, buffer and supported format; show independent cue capability; provide a safe low-level test tone only upon deliberate user activation; handle hot unplug, format mismatch and dropouts gracefully.

Recording, media decoding, waveform/beatgrid creation, transient analysis, stem separation, thumbnail generation, save and export run off the real-time callback. User has sole ownership of local media; no mandatory account, sync, remote AI or network connection for essential workflows.

## Instruments, plugins, production and mastering

- Planned native instrument and device family: oscillator/envelope/filter synth, sampler and drum pads, sequencer, loop capture, typed modular signal graph, granular/spectral processors and optional spatial audio.
- Planned integrated editing: piano roll, MPE when supported, audio/MIDI takes, comping, warp, fades, looping, automation, clip launcher, timeline, scene snapshots and versioned undo.
- Planned professional channel: trims, metering, EQ, dynamics, delay/FX, sidechains, sends, buses, crossfader routing, cue and monitor chain, plugin delay compensation and mastering export.
- Plugin hosting must survive failures where feasible. Evaluate CLAP and VST3 terms and host implementations before choosing formats. Scanner and untrusted plugin code are isolated when possible; process isolation adds IPC/latency. Unsupported plugins must be listed as such, never silently emulated.
- Mastering meters require calibrated reference signals. Export formats, channel layout, dither/bit depth, loudness and true peak are explicitly verified after rendering; "mastering ready" cannot be inferred from a mockup.

Optional AI tools are opt-in, privacy-preserving and run on background lanes. They must never sit on the live touch-to-audio critical path, delete recordings or upload stems without informed consent.

## Novel UX interaction systems to validate

**Fluid Session:** moving from performance to editing changes presentation, not project format. **Scratch Plane:** explicit perform vs navigate gesture layers, fine/coarse control, safe release behavior and device calibration. **Performance Memory:** capture both result audio and meaningful source gestures as editable automation with provenance. **Focus Morph:** task-oriented five-mode canvas sharing one graph, with frozen vital control positions in Stage Lock. **Hardware Independence:** all essential digital creative control tasks have touch/keyboard equivalents, while physical external devices remain optional.

All are hypotheses. Run DJ/producer comparative task tests before promoting them into marketing claims.

## Development ownership and acceptance

Conservative Rust evolution: existing engine, device and control crates remain; consider future modules for session schema/migrations, background media jobs, DAW editing/recording, host/plugin isolation and presentation shell only after each release gate. Do not scaffold every future module prematurely or modify unrelated repositories.

| Gate | Concrete proof required |
| --- | --- |
| DSP baseline | Deterministic four-deck PCM tests, offline master/cue isolation, strict Rust fmt/Clippy/build |
| Gesture priority | Queue saturation tests, safety/urgent input cannot starve, ordered scratch release, bounded callback time |
| Physical sound | Named OS/driver/desktop device, live listening, channel isolation, unplug/recovery, xrun logging |
| Touch scratch | Actual touchscreen input mapped to DSP, electrical loopback p50/p95/p99, independent expert DJ trial |
| Pro DJ vertical | Track import, complete routing, cues, loops, recording/reopen and 2-hour reference set |
| Canonical session | Save/reopen/undo/migration/corruption tests; transfer deck performance to arrangement and back |
| Integrated DAW | Record/edit/render a real song, instruments and supported plugins, fidelity checks and export |
| Production release | User acceptance, licensed source/deps, rights, installer/update/rollback and hardware matrix |

**Candidate performance targets (not achieved):** previous proposal p95 touch-to-sound no more than 20ms on explicitly supported wired hardware; explore under-10ms stretch only after instrumentation. Compare 64/128/256-frame buffers at 48kHz. Frame display may strive for 120Hz where supported but must have stable accessible reduced-motion and 60Hz/low-power operation.

**Evidence vocabulary:** DESIGNED (spec only); IMPLEMENTED (code present); CI VERIFIED (exact head passed named test); HARDWARE VERIFIED (named physical rig, raw result); DJ/PRODUCER VERIFIED (real user task); RELEASED (installable and reviewable). Never conflate CI with real audio or a public website with a deployed native app.

## Phase sequencing

0. Existing unmerged PRs #1-#3 engine/device/control; #4 identity; #5 full experience design.
1. Urgent/safety gesture lane and real device/loopback proof using current engine.
2. Four-deck practical DJ shell: safe load, playback, mixer, independent cue, scratching, full recording and error recovery.
3. Versioned canonical session, clip/arranger and performance-memory bridge with save/reopen.
4. Instruments, composition, modular effects, plugin host, mixing/mastering, rendering and advanced audio formats.
5. Professional hardware/OS certification matrix and continued release program with compatibility, security and accessibility regression.

No feature is complete until it passes its specific gate. Development can explore future UX in parallel, but production claims cannot advance ahead of verification. "Greatest DAW" is a mission benchmark, not a performance claim or fixed launch prediction.
