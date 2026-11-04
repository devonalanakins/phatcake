# Squelch — complete experience system

**Status:** End-state interaction design proposal, 2026-11-03. Product mission and no-mascot identity are owner-approved; the individual component layouts, behavior, visual details and technical implementation choices below remain reviewable design decisions. No playable DJ/DAW interface currently exists.

## One product, one continuous musical state

The user's work is a **Session**. Perform, Produce, Shape, Mix and Master are five task-focused views of the same session graph, transport and audio routing. Changing the view never duplicates a song, silently switches an audio source, modifies output routing, loses armed states or starts/stops a recording. Cross-view changes are explicit and undoable where safe. No application shell on the UI thread can determine audio callback timing.

The product must make it possible to complete three full journeys in one project:
1. **DJ:** import tracks -> analyze and prepare -> play/scratch four decks -> mix/cue -> record set -> reopen safely.
2. **Producer:** capture idea -> build instruments/MIDI/audio -> arrange -> mix -> master -> export -> reopen and reproduce.
3. **Hybrid:** improvise on four decks -> capture source tracks, faders and gesture automation -> transform those events into editable clips and arrangement -> audition on a deck without intermediate exports.

Journey 3 is the strategic product differentiation **hypothesis**, not a novel feature claim. Other DAWs already have performance-to-arrangement recording. Squelch must test whether touch-first scratching, hardware independence and one coherent source model produce a meaningfully better result.

## 1. Application shell, always visible

~~~text
┌──────────────────────────── SQUELCH ─────────────────────────────┐
│ session + save │ audio device / cue  │ CPU/xruns │ tempo │ stage lock │
│ PERFORM   PRODUCE   SHAPE   MIX   MASTER     [command/search]       │
├───────────────────────────────────────────────────────────────────┤
│ Library / Assets │         Adaptive Primary Canvas   │ Inspector    │
│ recent, search   │         one selected focus mode    │ track/device │
│ set lists        │         4-deck state persistent    │ context      │
├───────────────────────────────────────────────────────────────────┤
│ AUDIO CONTINUITY STRIP: transport | clock | record | cue | monitor  │
└───────────────────────────────────────────────────────────────────┘
~~~

- **Top status rail:** session title + save state + undo position, selected output device and count of independently addressable channels, headphone cue availability, actual sample rate/buffer, device failure, measured underrun count, tempo/time signature, transport state, stage lock and explicit recording indicator. No fabricated numerics: if unavailable show "not measured" or "device unavailable".
- **Five-mode navigation:** Perform, Produce, Shape, Mix, Master. Mode changes only change presentation/focus; no implicit audio command. Persistent keyboard shortcut and visible focus state; buttons are reachable by touch.
- **Library rail:** offline media, projects, set lists, samples, instruments, plugins, favorite devices and analysis jobs. Scales from simple search/favorites to metadata filters and bulk actions.
- **Context inspector:** selections, notes, routing, sample metadata, automation, undoable properties; can be pinned, collapsed or repositioned without hiding transport.
- **Audio continuity strip:** master output status, cue matrix, current session playback, recording, audio-engine health and panic/mute. In Stage Lock it becomes more prominent and cannot be overlapped by contextual menus.
- **Command palette:** search all actions by human-language label, keyboard shortcut and optional accessibility description; never sends arbitrary text straight to plugin or DSP engine.

## 2. Perform — four-deck professional cockpit

**Desktop (>=1280 CSS px):** two-by-two deck matrix around a four-channel vertical mixer; shared waveforms retain clear lanes, with a non-scrolling master transport, crossfader, cue matrix and accessible track browser.

**Landscape tablet (>=960 CSS px):** 2x2 four-deck status matrix + expandable single-deck gesture stage + mixer shelf; decks remain one action away and always communicate play/active/cue state.

**Compact tablet / phone (<960 CSS px):** persistent A/B/C/D overview strip, one enlarged selected-deck gesture surface, quick-swap deck selection and a dedicated mixer page/sheet; emergency mute, selected deck stop and master state stay visible. Never squeeze four full platter surfaces into a phone grid.

Each deck shows:
- Letter A/B/C/D plus accent and text label (not color alone); track title, artwork if rights allow, source time/beat phase/tempo/key with explicit analysis confidence; waveform overview and live contact zone; transport, cue, hot cues, loop, pitch/key, sync/slip and stem controls when implemented.
- A direct scratch surface with **gesture capture lock**. Press-and-hold does not page scroll; pinch zoom occurs only on a dedicated waveform navigation layer; multitouch fader input never unintentionally grabs the scratch zone. Gesture cancellation and finger-up behavior must be specified with neutral, release, motor/spring and braking models; all calibrated against real devices.
- Track replacement is a preloaded source swap only after an explicit performer decision, with unload and emergency interrupt protections. Do not read/decode audio or allocate unknown-sized buffers on the callback.
- Explicit output routing: crossfader side, cue enable, prefader meter, master send, postfader meter, headphone bus if device actually supports it.
- Three-tier feedback: direct control response; pending/accepted/rejected command acknowledgement; measured physical audio result where measurable. A UI animation is never evidence that sound changed.

**Stage Lock:** fixed positions for play, stop, mute and faders; hides potentially destructive dialogs and view rearrangement; preloads track and effects before a live set, permits explicitly whitelisted set actions; unlock requires deliberate action. This is workflow safety, not a guaranteed fail-safe. Critical mute must remain available with or without Stage Lock.

## 3. Produce — capture, compose and arrange

- **Canvas:** vertically grouped audio, MIDI, instrument, bus and performance-capture lanes sharing the session clock; time can display bars/beats and absolute samples/seconds, with explicit tempo-map conversions.
- **Dual composition:** linear arrangement and clip/scene launcher without duplicating device state; source media is referenced rather than destructively copied. Clip overrides and return-to-arranger behavior must be obvious.
- **Editors:** waveform trimming, fades, comping/take lanes, warping, piano roll, MPE expression editor where supported, groove editor, notation as an optional future exploration, automation and modulation lanes.
- **Performance capture:** record audio outputs or individual deck sources and capture control events/parameter automation with common timestamps. A recorded scratch gesture can become editable automation attached to a deck source; rendering to audio is explicit and stores lineage.
- **Undo and safety:** edits are non-destructive; media deletion from disk is separate from project removal; record-arm is explicit; crash recovery reconstructs last committed plus journaled edits and informs users of conflicts.

## 4. Shape — sound design and instrument canvas

- Three levels of depth: **Play** (macro controls, pads, presets) -> **Edit** (oscillators, envelopes, filters, sampler zones, modulation) -> **Patch** (typed node graph).
- Node sockets have clear data types (audio, MIDI/notes, control/CV-like modulation, event). Illegal cycles, feedback and mismatched channel graphs are validated before activation. Cross-node feedback requires bounded and documented delay.
- Sample slicing, granular and spectral effects are optional modules; destructive analyses are run outside the audio thread. Preview output is routed to an explicit audition bus.
- A playable XY pad or tactile instrument can be performed with the same input abstraction used by DJ scratching, but no uniform "velocity sensitivity" can be claimed on devices lacking pressure/haptics.
- Presets are versioned and diffable; missing samples/plugins open in bypass or relink mode with clear fidelity warnings, rather than silently replacing the sound.

## 5. Mix — routing and engineering control

- Configurable channel strips with meter, gain, trim, pan, sends, returns, fader, mute/solo/cue, EQ, dynamics, inserts, sidechain and channel grouping.
- Routing graph has explicit edges, channel maps and signal order; meters distinguish pre/post points. Monitor latency, plugin processing latency and channel overload are measured or marked unknown.
- Mix snapshots and A/B comparisons can be auditioned without irreversible overwrite; hardware controllers are optional bidirectional mappings.
- During an active show, arbitrary graph rewiring is prevented or deferred until a safe transition. Emergency mute can bypass non-critical UI locks.

## 6. Master — inspect, compare, deliver

- Mastering chain view with loudness/true-peak/spectrum/correlation/dynamics tools where implemented, delivery presets, reference comparison and export queue.
- Unknown readings display "-- / unavailable" rather than demo numbers. Actual LUFS/true-peak compliance is verified against reference signals and post-export analysis.
- Exports have exact source range, sample format, bit depth, channel mapping, dithering policy and metadata review; queue remains inspectable and cancellable with partial files cleaned.
- Before a final export, show unsaved state, bypassed/missing plugin and clipping/limiter flags. Output a render report tied to the saved session version.

## 7. Cross-cutting input model and interaction grammar

- **Input normalization:** keyboard, pointer, touchscreen, stylus, optional MIDI/MPE/OSC/controllers feed typed actions with source/device identity and timestamps; actions are authorized, scoped and admitted into the appropriate control lane.
- **Direct-manipulation priorities:** touching audio changes an explicit property; dragging a clip never alters routing; two-finger gestures do not modify output by accident; hold-to-fine-adjust on faders with visible value increments and alternative keyboard entry.
- **Selection model:** click/tap for single, modifier/multiselect, long-press contextual command; consistent across waveform, timeline, piano roll and node graph. On touch, require explicit edit vs perform mode when ambiguity risks an accidental cut or scratching.
- **Precision:** quantization is optional and visible; scratch is never silently snapped to grid. Transport is quantized only when the selected action states so.
- **Search and command grammar:** one global palette with paths such as "Add drum instrument", "Route Deck C cue", "Export stems", "Show CPU budget". Every result indicates whether it changes audio, metadata or display.
- **Offline first:** no account required for primary work; any local AI assistant is opt-in and offline/remote status is explicit; no surprise indexing or transfer of private recordings.
- **Localization readiness:** text can expand, controls do not depend on English words baked into imagery and notation settings are per user.
- **Motion & accessibility:** keyboard complete for critical functions, screen reader labels/operable transport, visible focus, reduced-motion, forced-colors/high-contrast mode, no 3D dependence for waveform interpretation. WCAG 2.2 AA is the intended UI design audit target, not a certification claim.

## 8. Responsive/reflow rules (initial targets, not device support promises)

| Window | Behavior | Critical guarantee |
| --- | --- | --- |
| >=1600 px | Four large deck surfaces, center mixer, library and inspector | All four play/cue states visible |
| 1280-1599 px | Compact 2x2 decks, central mixer or collapsible inspector | Transport and cue matrix fixed |
| 960-1279 px | Four-deck overview, enlarged active deck and mixer shelf | One-tap deck selection, no hidden master |
| 600-959 px | Selected gesture deck, persistent four-deck status, modal inspector | Record and mute status always visible |
| <600 px | Four-deck compact overview, focused deck, dedicated mixer route | Phone is a reduced-density workflow, not a guarantee of professional parity |

Touch targets: prefer 48 CSS px or larger for primary controls; validate physical sizes on target devices. Text, scrollable zones, safe-area insets and horizontal overflow must be measured on actual iOS/Android devices before claiming support.

## 9. Onboarding and recovery

1. Create or open local session; choose intended use (Perform / Produce / Hybrid) **as layout**, not a different file type.
2. Guided device check: output routing, sample rate/buffer, channel count, separate cue route verification, headphones confirmation and test signal at a safe controlled level with explicit user action.
3. Optional input calibration: scratch touch surfaces, tactile preference, fader sensitivity and accessible key mappings.
4. Load authorized sample media, inspect analysis provenance and set-list readiness. Do not download music or assert licensing rights on behalf of the user.
5. Failure states: device unplug, plugin crash/hang, dropped buffers, absent cue outputs, incomplete analysis, missing media, unsaved edits, underrun and recording interruption each have a distinct diagnostic + safe remediation. Avoid modal popups that conceal emergency controls.
6. Startup and update flow is non-destructive; update migration preview, backups and recovery before applying a file-schema migration.

## 10. Task journeys to usability-test

| Journey | Completion evidence |
| --- | --- |
| New DJ, no controller | Open sample -> choose output -> cue -> load 4 decks -> scratch -> record and reopen |
| Pro DJ 2-hour set | Set lock -> route independent cue -> perform 2 h -> recover from controlled device reconnect |
| Producer full song | Create instrument, MIDI and audio -> arrange -> mix -> master -> export -> reopen bitwise/or audibly equivalent |
| DJ to producer | Capture scratch + other tracks as timed clips -> edit in arranger -> reload modified content on deck |
| Accessibility | Complete transport, cue, routing, recording and export using keyboard and assistive output |
| Failure recovery | Inject plugin crash/asset loss/underrun without session corruption or hidden audio routing |

Only mockup navigation and cosmetic controls may be demonstrated by the design prototype. All audio claims require hardware/device testing.

## 11. Review state

**LOCKED:** product name Squelch, unified DAW destination, no pet/mascot, dedicated creative control hardware optional by design, four independent decks minimum, stage reliability must not be traded for animation.

**PROPOSED:** five-mode shell visual treatment, stage lock behavior, component layout, exact breakpoints, feature arrangement and editable performance capture metadata. This document is an interaction contract candidate.

**NOT IMPLEMENTED:** user-facing interface, audio-connected deck controls, session graph, DAW, plugin host, recording, rendering, physical measurement, network features. Refer to the five upstream draft PRs and actual CI rather than mockups for code evidence.
