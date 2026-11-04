# Squelch — proposed visual design system, v0.1

**Visual direction proposal — not a locked final brand palette or installed font package.** Use this as a consistent baseline for concept reviews and future accessible components.

## Personality

Professional performance instrument, with high information density organized into calm hierarchies. Precise tactile surfaces, luminous signal traces and restrained depth. No hardware cosplay, cartoon mascots, cake imagery, toy-like skeuomorphism or ornamental sci-fi elements that obscure audio data. The UI feels like an instrument: direct, dependable, expressive and immediately readable at arm's length.

## Colors and status rules

| Token | Hex | Role |
| --- | --- | --- |
| canvas | #090D14 | Main workspace |
| canvas-secondary | #0F1622 | Editing background |
| panel | #151F2D | Deck / mixer modules |
| raised | #203047 | Hover, selected surfaces and popovers |
| border | #314357 | Structural edges, never only separator of control |
| primary-text | #EFF5FB | Titles, readouts |
| secondary-text | #B2C2D3 | Metadata and explanations |
| subdued-text | #8EA1B3 | Decorative labels; not for critical transport readouts |
| signal | #D7F564 | Selected primary focus/transport action |
| deck-a | #B990FF | A waveform/focus + literal "A" |
| deck-b | #FFB068 | B waveform/focus + literal "B" |
| deck-c | #75E4B4 | C waveform/focus + literal "C" |
| deck-d | #6CCBFF | D waveform/focus + literal "D" |
| alert | #FF8585 | Fault states with icon and text |
| warning | #FFD58E | Latency/clip/cue warnings with copy |
| valid | #9EEBA9 | Verified route/ready status with copy |

High-contrast and color-vision tests must be run before claiming WCAG conformance; exact deck hues may change. Reduced luminosity for dark-club use must not make important values faint. Signal yellow is not a blanket replacement for all status colors.

## Type scale

Use system-ui or a permissively licensed geometric sans vetted for UI numerals and multilingual glyphs. Use a monospaced font for timestamps, BPM, dB, sample counts and status codes. Do not ship a proprietary font or copy a competitor's typeface without rights.

- Hero/brand: 28-36px, semibold, not used for live meters.
- Workspace headers: 19-24px.
- Component names: 12-15px strong, letter spacing only for short labels.
- Primary numerics and transport values: 15-24px with tabular numerals.
- Body: 14-16px minimum; essential failure labels never rely on 10px microtype.
- Dense but legible engineering metadata: 12-13px, with scaling and tooltip/accessible expansion.

## Spacing, dimensions and elevation

- 4px rhythm; main shell content gap 16-20px; internal module gap 8-12px.
- Key interaction controls target at least 48x48 CSS pixels with invisible extra hit area when practical. Performance safety controls are larger.
- Radii 8px for tactile elements, 12px deck panels, 16-20px focus surfaces; no generic oversized pills on all controls.
- Borders 1px; layered depth through subtle internal gradients and selective glow **only** for active/selected and no more than one focus state.
- All shadows and blur are optional decorative rendering, never needed for meaning.

## Audio visualization rules

- Waveforms: samples/time on x axis; signed amplitude on y axis; show a neutral midline and bar/beat markers only if real beatgrid metadata exists. Render overview and zoom views separately.
- Spectrograms: mark Hz and time, and show frequency range; don't invent frequency heat maps while no audio is loaded.
- Meters: use measured peak/RMS or specified metering, with units and clipped/hold indicator; never use moving bars as simulation in production UI without a DEMO indicator.
- Scratch cursor: show actual input capture/finger and commanded frame position; separate callback-accepted position and measured hardware presentation where supported.
- Stage mode uses less motion, brighter focus rings and persistent mute/record icons.

## Interaction component catalog

1. DeckTile — ID, loaded media/title, waveform, transport, cue, active output, xrun/ready state, selected focus.
2. ScratchPlane — contact target, safe gesture capture, direction/velocity visual feedback, release mode and calibrated sensitivity.
3. ChannelStrip — gain, EQ, sends, cue, mute/solo, metering, fader, source assignment, focus and value popup.
4. MasterSection — output device/channels, master meter, output trim, emergency mute, cue bus and crossfader.
5. ClipBlock — type, timeline start/end, live/loop/armed, take lane, local asset status and edit affordances.
6. InstrumentModule — preset, macro controls, patch view, modulation graph, audition routing.
7. SignalNode — typed ports and validated edges, loop detection and latency warning.
8. Inspector — properties, routing, processing chain, analysis evidence and undo-safe edits.
9. StatusToast — persistent failures require non-toast diagnostic entry and may not obstruct transport.
10. CommandPalette — actions with keyboard label, read-only vs mutating badges and scope.
11. DeviceCheck — routing matrix, supported sample rates, output channel isolation and safe test procedure.
12. RenderQueue — format/rate/bit depth, exact range, license-rights reminders when relevant, progress, failures/retry.
13. StageLock — fixed layout, destructive action suppression, clear unlock and fail-safe cue/mute access.

## Interactive and motion spec

- Hover: informational only, no transport changes.
- Press: immediate visual acknowledgement; audio command acknowledgement comes separately from engine, with pending/error state.
- Drag/fader: direct and coarse/fine selectable; no spring animations that hide the actual numeric value.
- Scene and mode changes: <=220 ms optional transition if rendering budget allows; reduced-motion = immediate swap.
- Graph transitions: precompute visual layout off real-time thread; optional 3D/audio visualizations are separate panels, not required.
- Loading: distinguish seeking/decoding/analysis progress; reject unsupported file formats visibly and preserve session.
- Undo: mutating metadata operations transactional; transport operations typically not undoable; print explicit examples in tooltips.

## Design prototype and truthfulness

See [interactive browser-only concept](../design/squelch-ui-prototype.html). It has no dependency on the Rust audio engine and produces **no sound**. Its waveforms are procedural decorative demo signals, not calculated from real media. Controls demonstrate layout and local UI state only; no real session persistence, audio routing, input measurement or instrument synthesis. Never present it as a functional music application.

## External design validation

- Independent creator, experienced scratch DJ, studio producer, accessibility participant and live engineer should evaluate distinct tasks, not only aesthetic satisfaction.
- Review at 320/390/768/1024/1440/1920 CSS pixel widths, screen zoom 200%, mobile safe-area, light/dark outdoor conditions, focus-only navigation, reduced motion and screen-reader semantics.
- Track discoverability, wrong-target rate, latency perception (with real sound), error recovery and task completion. Do not equate a concept screenshot with user validation.
