//! Device-independent interleaving for Squelch's four-deck audio engine.
//! The render method allocates no memory, locks no mutex, and performs no I/O.

#![forbid(unsafe_code)]

use squelch_engine::{Engine, EngineError, StereoFrame, StereoRenderer};

const BLOCK_FRAMES: usize = 128;
const STEREO_SAMPLES_PER_BLOCK: usize = BLOCK_FRAMES * 2;

/// Errors from invalid output data or a failed DSP render.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputError {
    InvalidLayout,
    Engine(EngineError),
}

/// Fixed-size buffers owned exclusively by one audio callback.
#[derive(Debug)]
pub struct OutputScratch {
    master: [f32; STEREO_SAMPLES_PER_BLOCK],
    cue: [f32; STEREO_SAMPLES_PER_BLOCK],
}

impl Default for OutputScratch {
    fn default() -> Self {
        Self {
            master: [0.0; STEREO_SAMPLES_PER_BLOCK],
            cue: [0.0; STEREO_SAMPLES_PER_BLOCK],
        }
    }
}

impl OutputScratch {
    pub fn new() -> Self {
        Self::default()
    }

    /// Output 1/2 is stereo master. Output 3/4, when present, is stereo cue.
    /// Remaining device channels are written as silence.
    ///
    /// The engine and scratch buffers must never be accessed concurrently.
    pub fn render(
        &mut self,
        renderer: &mut impl StereoRenderer,
        output: &mut [f32],
        channels: usize,
    ) -> Result<(), OutputError> {
        if channels < 2
            || !output.len().is_multiple_of(channels)
            || channels.checked_mul(BLOCK_FRAMES).is_none()
        {
            output.fill(0.0);
            return Err(OutputError::InvalidLayout);
        }

        let block_samples = channels * BLOCK_FRAMES;
        for block in output.chunks_mut(block_samples) {
            let frames = block.len() / channels;
            let stereo_samples = frames * 2;

            if let Err(error) = renderer.render_into(
                &mut self.master[..stereo_samples],
                &mut self.cue[..stereo_samples],
            ) {
                block.fill(0.0);
                return Err(OutputError::Engine(error));
            }

            for (index, frame) in block.chunks_exact_mut(channels).enumerate() {
                let offset = index * 2;
                frame.fill(0.0);
                frame[0] = self.master[offset];
                frame[1] = self.master[offset + 1];
                if channels >= 4 {
                    frame[2] = self.cue[offset];
                    frame[3] = self.cue[offset + 1];
                }
            }
        }

        Ok(())
    }
}

/// Generate four one-second looping sine waves before starting playback.
/// No copyrighted music or network access is needed to test device output.
pub fn prepare_demo_engine(sample_rate: u32) -> Result<Engine, EngineError> {
    let mut engine = Engine::new(sample_rate)?;
    let frequencies = [110.0_f32, 165.0, 220.0, 330.0];

    for (index, frequency) in frequencies.iter().copied().enumerate() {
        let pcm: Vec<StereoFrame> = (0..sample_rate)
            .map(|sample| {
                let phase = std::f32::consts::TAU * frequency * sample as f32 / sample_rate as f32;
                let value = phase.sin() * 0.06;
                StereoFrame::new(value, value)
            })
            .collect();

        let deck = engine.deck_mut(index).expect("four-deck engine invariant");
        deck.load_pcm(pcm)?;
        deck.set_loop(0, sample_rate as usize)?;
        deck.set_cue_enabled(index == 1);
        deck.play()?;
    }

    Ok(engine)
}
