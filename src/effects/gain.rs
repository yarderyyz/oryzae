use crate::network::{
    BlockRequirements, BlockSize, NetworkRealReal, ProcessStatus, RealBuffer, RealBufferMut,
};

/// Gain/volume control effect
///
/// Multiplies all input samples by a gain factor. Can be used for
/// volume control, attenuation, or amplification.
pub struct Gain {
    gain: f64,
    out_frame: Vec<f64>,
}

impl Gain {
    /// Create a new gain effect
    ///
    /// # Arguments
    /// * `gain` - Gain multiplier (1.0 = unity gain, 0.5 = half volume, 2.0 = double volume)
    ///
    /// # Returns
    /// * Boxed Gain instance
    pub fn new(gain: f64) -> Box<Self> {
        Box::new(Self {
            gain,
            out_frame: vec![],
        })
    }

    /// Set the gain value
    ///
    /// # Arguments
    /// * `gain` - New gain multiplier
    pub fn set_gain(&mut self, gain: f64) {
        self.gain = gain;
    }

    /// Get the current gain value
    ///
    /// # Returns
    /// * Current gain multiplier
    pub fn get_gain(&self) -> f64 {
        self.gain
    }
}

impl NetworkRealReal<f64> for Gain {
    fn process(&mut self, input: RealBuffer<f64>, output: RealBufferMut<f64>) -> ProcessStatus {
        if input.is_empty() || output.is_empty() {
            return ProcessStatus::Ready;
        }

        let channels = input.len().min(output.len());
        for ch in 0..channels {
            let samples = input[ch].len().min(output[ch].len());
            for i in 0..samples {
                output[ch][i] = input[ch][i] * self.gain;
            }
        }

        ProcessStatus::Ready
    }

    fn block_size(&self) -> BlockRequirements {
        BlockRequirements {
            input_size: BlockSize::Flexible,
            output_size: BlockSize::Flexible,
        }
    }
}
