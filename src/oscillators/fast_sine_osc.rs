use crate::network::{
    BlockRequirements, BlockSize, NetworkRealReal, ProcessStatus, RealBuffer, RealBufferMut,
    SYSTEM_SAMPLE_RATE,
};

/// Sign enum for FastSineOsc phase tracking
#[derive(Debug, Clone, Copy)]
pub enum Sign {
    Positive,
    Negative,
}

impl Sign {
    pub fn flip(&self) -> Sign {
        match self {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
        }
    }
}

/// Fast sine wave oscillator using parabolic approximation
///
/// Fast Sine is good for a cheap simple LFO, it runs at audio rates fine but could not be
/// remotely described as a "clean" sine oscillator. Uses a parabolic approximation for speed.
pub struct FastSineOsc {
    phase: f64,
    phase_increment: f64,
    sign: Sign,
    out: Vec<f64>,
}

impl FastSineOsc {
    /// Create a new fast sine oscillator
    ///
    /// # Arguments
    /// * `frequency` - Frequency in Hz
    ///
    /// # Returns
    /// * Boxed FastSineOsc instance
    pub fn new(frequency: f64) -> Box<Self> {
        Box::new(Self {
            phase_increment: frequency * 4.0 / SYSTEM_SAMPLE_RATE.get().unwrap(),
            phase: -1.0f64,
            sign: Sign::Positive,
            out: vec![0.0],
        })
    }
}

impl NetworkRealReal<f64> for FastSineOsc {
    fn process(&mut self, _input: RealBuffer<f64>, output: RealBufferMut<f64>) -> ProcessStatus {
        if output.is_empty() || output[0].is_empty() {
            return ProcessStatus::Ready;
        }

        let current_phase = self.phase;

        let sample = match self.sign {
            Sign::Positive => current_phase * current_phase - 1.0,
            Sign::Negative => -(current_phase * current_phase) + 1.0,
        };

        let mut new_phase = current_phase + self.phase_increment;
        if new_phase >= 1.0 {
            new_phase -= 2.0;
            self.sign = self.sign.flip();
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
