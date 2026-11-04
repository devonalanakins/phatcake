# Squelch — unified music creation and performance vision

**Decision status:** The owner has set the long-term product ambition: build a continually improved, fully capable digital audio workstation (DAW) and DJ/live-performance environment that can replace the **creative/control functions** of dedicated music hardware wherever technically feasible. This is **a product objective**, not implemented functionality or a substantiated claim of market superiority.

**Product identity:** Squelch. No pet, mascot, cartoon, companion character, cake motif or unrelated game features.

## Product thesis

**One session. From playing music to making it to mastering it.**

Squelch should remove the traditional boundaries among DJ software, instruments, drum machines, samplers, multitrack recording, MIDI composition, sound-design environments, live looping, mixing consoles, effects racks, mastering suites and performance control surfaces.

The differentiator must be an integrated experience, not a collection of tabs that imitate other applications. Users should be able to scratch a deck, capture the live gesture and its audio, turn it into a clip or track, modify the sound in an instrument/effects graph, arrange a song, export masters, then bring that production back onto a deck **without changing projects or translating between incompatible session models**. This is a design target; no such transfer exists in the current implementation.

## Hardware-independent by default: precise meaning

Squelch's own software controls are intended to make a dedicated DJ controller, mixer, turntable, drum pad, synthesizer keyboard, sequencer, groovebox, outboard effects unit and control surface **optional rather than required** for equivalent digital control and processing tasks, subject to validated performance and input compatibility.

Software does **not** replace physical acoustic and conversion requirements. A computer or other compute device, some DAC/output path, speakers/headphones, and microphones/ADCs for real-world recording remain necessary when their respective physical functions are needed. A single stereo output cannot provide genuinely isolated master and headphone cue simultaneously without separate physical outputs; Squelch must present routing availability honestly. A touchscreen is not inherently identical to motorized platter resistance, physical haptics or every professional control feel. External equipment remains supported as an optional extension and as redundancy for critical live shows.

## Complete end-state capability map (requirements, not features already built)

1. **Performance/DJ:** four or more independent simultaneous decks, platter and waveform scratching, accurate reverse and slip, hot cues, looping, beatgrid analysis/editing, variable tempo/key, harmonic mixing, flexible crossfading/routing, stem control, effects, robust library, set preparation, recording and recovery.
2. **Recording/DAW:** multitrack audio and MIDI, non-destructive editing, take lanes and comping, piano roll and MPE support where possible, quantization and humanization, time-warp/stretch, tempo maps, unlimited logical tracks subject to hardware capacity, clip launcher, linear arranger, scene snapshots, automation, export and project portability.
3. **Instruments:** integrated synthesizers, sampled instruments, drum machines/pads, samplers, loopers, sequencers, patchable modular signals, granular/spectral options and expressive gesture mappings.
4. **Mixing/mastering:** professional EQ, dynamics, sends/returns, flexible buses and channel routing, meters, sidechain, delay compensation, effects rack, mastering analysis, limiting, loudness and stems/multiformat export. Do not claim compliance with a broadcast delivery standard until verified.
5. **Interoperability:** project-local media and open documented formats; plugin hosting (investigate CLAP first, VST3 per the current SDK terms), MIDI/MPE where technically supported, optional controller integration, multichannel audio I/O, batch render, session import/export with fidelity testing.
6. **Stage/expanded creation:** live performance scenes and automation, potential video/visual sync, spatial audio, network collaboration, optional AI-assisted creative tools, and future protocols for external stage equipment. These are backlog hypotheses, not first-release promises.

## Non-negotiable experience principles

- **Perform / Produce / Shape / Mix / Master** are focus surfaces over **the same session data and audio graph**. Transition must preserve armed tracks, decks, routing, transport and undo history. A performance can become an arrangement; a produced clip can become a deck source.
- **Four-deck identity always available** in Perform; other surfaces retain a compact performance-state rail when needed. Never hide a critical master/cue/stop operation behind visual navigation.
- **Direct manipulation before imitation:** gesture-sensitive waveforms, touch-optimized faders with fine adjustment, configurable scratch release and inertia, object-centered selection, and optional keyboard/physical controller equivalents. Avoid merely drawing skeuomorphic hardware with tiny controls.
- **Fluid canvas, predictable controls:** contextual editing panels, resizable modular surfaces, zoom between timeline, track, clip and DSP graph, and adaptive layouts. In a locked live-performance state, primary buttons must not move dynamically and surprise a performer.
- **No audio compromise for graphics:** UI can be cinematic and responsive, but effects and motion run outside audio callbacks and degrade gracefully; reduced-motion mode and keyboard/screen-reader equivalents are required.
- **Explain reality:** device support, active cue routing, clip readiness, plugin health, dropouts, render latency, unsaved changes and hardware limits are shown from measured signals, never fabricated demo telemetry.
- **Offline-first independence:** music and project files local by default; no mandatory account, cloud round trip or AI network call on the live audio path. Optional sync/collaboration requires explicit authorization, versioning and privacy boundaries.

## Shared technical platform (proposed architecture)

- **Canonical session graph:** versioned, interoperable representation for deck sources, clips, audio/MIDI tracks, transport, tempos, automation, instruments, effects, buses, cue/master routing, snapshots, assets and metadata. Design this **now**, even though its production implementation will follow validated DJ audio fundamentals.
- **Realtime DSP engine:** single-owner or carefully partitioned audio render graph with bounded scheduling, no unbounded heap allocation/locks/network on the audio callback, sample-rate-aware processing, latency compensation, recovery and diagnostics.
- **Control plane:** high-priority, bounded performance events separated from scheduled automation; deterministic arbitration, exact-frame semantics where supported, panic/transport safety and observable backpressure.
- **Media/background domain:** decoding, waveform/spectrogram analysis, stem generation, indexing, plugin discovery and project save/recovery off audio threads.
- **Plugin host:** investigate CLAP and VST3 formats with their current terms; isolate misbehaving untrusted plugins where possible, avoid hangs/crashes in live playback, disclose plugin bridge latency and rights; do not promise universal third-party-plugin compatibility.
- **Presentation/input domain:** native touch/pointer/keyboard and optional external controllers map to the same typed commands, with bounded admission into DSP. UI redraw frequency must not define scratch/audio timing.
- **System portability:** prioritize desktop-class Windows/macOS/Linux platforms after platform qualification; mobile/tablet builds adapt to actual OS I/O, scheduling and plugin constraints. Do not imply full desktop plugin hosting on all mobile devices.
- **Session durability:** non-destructive operations, autosave/snapshot, crash recovery, media relinking, schema migrations, import/export tests, resource budgets and safe failure modes.

## Performance and proof standard

"Most advanced" and "greatest" are **ambitions, not product claims**. Benchmark against current leading DJ and DAW software and published user workflows, using comparable devices and measurable acceptance criteria rather than subjective assertions.

Minimum validation categories: four-deck transport correctness; independently isolated cue/master; scratch gesture quality against real human DJ testing; loopback-measured touch-to-sound p50/p95/p99 latency on declared devices; audio underrun count under CPU/GPU stress; project save/reopen fidelity; plugin crash resilience; 2-hour-plus continuous stability; accessible and usable touch/keyboard interfaces; cancellation/undo of destructive edits; and export accuracy. Targets must be documented per operating system and device and revised on evidence. Previous candidate latency and frame-rate numbers are unverified targets.

## Sequenced delivery, unified destination

- **Foundation (current prototype):** existing four-deck offline Rust DSP, experimental native CPAL output and bounded scheduled control. Existing stacked PRs #1-#5 are the engineering source of truth; no DAW functionality or playable touch UI is established.
- **Gate A — trustworthy hardware-free digital DJ controls:** urgent gesture lane, real wired audio testing, four-deck transport, scratch prototype and measured latency. A microphone or multichannel interface may still be needed for external capture and independent cue outputs.
- **Gate B — professional DJ vertical slice:** track import/library, beatgrid, scratch/loop/cue, mixer, recording, saved sets, accessible adaptive four-deck UI; repeatedly perform real DJ user tests.
- **Gate C — shared session/DAW core:** unified track/clip/scene graph, audio+MIDI recording and editing, automation, save/reopen and transfer between decks and arranger. The architecture is planned from day one, but the implementation is released incrementally.
- **Gate D — complete workstation:** built-in instruments, sampler/drum systems, production editing, modulation, plug-in hosting, mixer/mastering and reliable render/export. Feature completeness must be compared against an explicit reference matrix, not assumed.
- **Gate E — verified differentiated innovation:** context-adaptive performance canvas, direct high-resolution gesture controls, performance-to-song capture, optional advanced spatial/AI/network functions, expansion tools and a long-lived compatibility program.

Release and spending remain subject to the owner's approval and project licensing decisions. No fixed completion date or claim of inevitability. Every release needs backward compatibility, rollback, automated tests, real-device validation and a clear changelog.

## Competitive reality, not a novelty claim

Existing DAWs already combine clip launching, arrangement, modulation and multitouch (e.g., Bitwig and Ableton), while Traktor and djay offer mature four-deck performance. Squelch's proposed advantage must be demonstrated through continuity between these workflows, better direct-control interaction, verified real-device performance and hardware independence. Do not claim any single listed feature is unprecedented.

Background product references:
- https://www.bitwig.com/overview/
- https://www.ableton.com/en/live/all-new-features
- https://www.native-instruments.com/products/traktor-pro
- https://www.algoriddim.com/djay-pro-windows
- https://u-he.com/community/clap/
