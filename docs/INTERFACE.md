# Squelch — touch-first performance cockpit

**Status:** Design proposal only. The current Rust code contains offline four-deck DSP, an experimental native output adapter and a bounded scheduled control queue. There is no playable touchscreen UI, measured touch-to-sound latency or full scratch engine.

**Unified experience:** This cockpit is the **Perform** surface of the longer-term Squelch DAW, not a separate DJ product. Proposed Produce, Shape, Mix and Master surfaces operate on one persistent session. See [unified product vision](VISION.md).

## Performance architecture

```text
    DECK A          MASTER / CUE MIXER           DECK B
    waveform    level · crossfader · meters       waveform
    transport      urgent mute / cue            transport
    pitch / loop     routing states           pitch / loop

    DECK C       SYSTEM / AUDIO DEVICE            DECK D
    waveform     driver · channel status          waveform
    transport       xrun diagnostics            transport
    pitch / loop   timer / recording*          pitch / loop

    * future capability; not currently implemented
```

Desktop and landscape tablets should show all four deck identities and play states simultaneously. For narrower tablets, preserve four deck status tiles while enlarging the selected tactile surface. Phones can use a compact four-deck overview and focus on one detailed scratch editor; do not pretend a phone can fit desktop-sized precision controls.

## Controls

- Critical controls always discoverable: play, pause, cue, gain, output channel, headphone cue and deck A/B/C/D state.
- Touch scratch surface captures gestures intentionally without accidental page scrolling. The gesture-to-sound path must be measured under stress, not assumed from visual framerate.
- Transport and emergency mute must have an immediate priority lane. Existing single FIFO scheduling cannot guarantee urgency when future events occupy it.
- Track status is explicit: unloaded, ready, playing, paused, channel unavailable, clipping or underrun. Avoid aesthetic waveform animations that imply fabricated audio signals.
- Cue and master routing use labeled signals and independent meters; headphone cue must not be confused with master output.
- Keyboard and accessibility alternatives accompany touch controls, including focus, labels, contrast and reduced-motion behavior.

## Technical sequencing

```text
  Multi-touch / keyboard
        |
  High-priority gesture arbitration [NOT IMPLEMENTED]
      |                       |
  Urgent transport          Scheduled automation
      \                       /
       Sample-time audio renderer [PROTOTYPE]
                 |
         Native audio adapter [EXPERIMENTAL]
            |             |
          MASTER         CUE
```

1. Implement bounded urgent gesture arbitration and test future automation cannot block touch transport.
2. Verify real audio devices: stereo master and >=4-channel master-plus-cue with headphones; measure underruns.
3. Prototype one genuine responsive touch deck without inventing latency numbers.
4. Record event -> enqueue -> callback -> driver/output loopback timings and p50/p95/p99 under CPU and GPU stress.
5. Expand to four visible decks and harden keyboard/focus/reduced-motion, portrait/landscape tablet and phone layouts.
6. Gate release on licensing, rights for music files, driver matrix, stability, packaging and independent review.

Audio callbacks must remain independent of rendering, networking and storage. DAW production, recording, arrangement and cloud workflows are deferred directions, not promises.

## Context-adaptive studio canvas (long-term design proposal)

The four-deck matrix is a persistent Perform cockpit, but Squelch must also support a full DAW. A shared session should allow a deck to become an editable clip, a live mix to become multitrack session content, and a composed track to be loaded into a deck without exporting between incompatible projects. These transitions are **not built**.

| Focus surface | Primary task | Persistent context |
| --- | --- | --- |
| **Perform** | Four-deck scratching, cues, loops, scenes and mixer | Master/cue state, performance lock, transport |
| **Produce** | Record/edit audio and MIDI, arrangement, clips, automation | Current tracks, tempo map, shared clip/deck sources |
| **Shape** | Synths, samplers, drums, modular signal graph, FX | Current signal chain, modulation and audition state |
| **Mix** | Buses, sends, dynamics, EQ, automation, channel routing | Live output state, safe gains and headroom |
| **Master** | Loudness, final limiting, versions and export | Source reference, delivery target and measured meters |

### Interaction invariants

1. Any mode transition preserves audible output, cue/master isolation, transport and undo/snapshot history.
2. During a locked live set, the locations of critical play/stop, cue and gain controls must not shift automatically.
3. Gesture controls include fine/coarse adjustment, explicit capture regions, calibration, keyboard alternative and reduced motion.
4. Every fader and knob displays real values, not only attractive animations; dangerous edits offer explicit limits and feedback.
5. The UI can be visually dramatic but must remain performant on declared hardware with a fully usable static/low-power mode.
6. Creative tools or AI assistance never block real-time processing or silently overwrite recorded work.

**Design acceptance:** Validate both an expert four-deck DJ workflow and a complete record→edit→mix→export song workflow. The second remains a future release gate; a cinematic mockup alone does not satisfy it.
