use crate::network::{
    BlockRequirements, BlockSize, NetworkRealReal, ProcessStatus, RealBuffer, RealBufferMut,
    SYSTEM_SAMPLE_RATE,
};
use std::f64::consts::PI;

/// High-quality sine wave oscillator
///
/// Generates clean sine waves using the standard sin() function.
/// Suitable for audio-rate synthesis where quality is important.
pub struct SineOsc {
    phase: f64,
    phase_increment: f64,
    out: Vec<f64>,
}

impl SineOsc {
    /// Create a new sine oscillator
    ///
    /// # Arguments
    /// * `frequency` - Frequency in Hz
    ///
    /// # Returns
    /// * Boxed SineOsc instance
    pub fn new(frequency: f64) -> Box<Self> {
        Box::new(Self {
            phase_increment: frequency * 2.0 * PI / SYSTEM_SAMPLE_RATE.get().unwrap(),
            phase: 0.0f64,
            out: vec![0.0],
        })
    }
}

impl NetworkRealReal<f64> for SineOsc {
    fn process(&mut self, _input: RealBuffer<f64>, output: RealBufferMut<f64>) -> ProcessStatus {
        if output.is_empty() || output[0].is_empty() {
            return ProcessStatus::Ready;
        }

        let current_phase = self.phase;
        let sample = current_phase.sin();
        let mut new_phase = current_phase + self.phase_increment;
        if new_phase >= 2.0 * PI {
            new_phase -= 2.0 * PI;
        }
        self.phase = new_phase;
        output[0][0] = sample;
        ProcessStatus::Ready
    }

    fn block_size(&self) -> BlockRequirements {
        BlockRequirements {
            input_size: BlockSize::Fixed(1),
            output_size: BlockSize::Fixed(1),
        }
    }
}
