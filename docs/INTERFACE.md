# Squelch — touch-first performance cockpit

**Status:** Design proposal only. The current Rust code contains offline four-deck DSP, an experimental native output adapter and a bounded scheduled control queue. There is no playable touchscreen UI, measured touch-to-sound latency or full scratch engine.

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
