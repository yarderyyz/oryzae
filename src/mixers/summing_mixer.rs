use crate::network::{
    BlockRequirements, BlockSize, NetworkRealReal, ProcessStatus, RealBuffer, RealBufferMut,
};

/// Summing mixer that adds all input channels together
///
/// Takes multiple input channels and sums them into a single mono output channel.
/// Useful for combining multiple audio sources.
pub struct SummingMixer {
    out_frame: Vec<f64>,
}

impl SummingMixer {
    /// Create a new summing mixer
    ///
    /// # Returns
    /// * Boxed SummingMixer instance
    pub fn new() -> Box<Self> {
        Box::new(Self {
            out_frame: vec![0.0],
        })
    }
}

impl NetworkRealReal<f64> for SummingMixer {
    fn process(&mut self, input: RealBuffer<f64>, output: RealBufferMut<f64>) -> ProcessStatus {
        if input.is_empty() || output.is_empty() || output[0].is_empty() {
            return ProcessStatus::Ready;
        }

        let samples = input.iter().map(|ch| ch.len()).min().unwrap_or(0);
        let max_samples = output[0].len().min(samples);

        for i in 0..max_samples {
            output[0][i] = input.iter().map(|ch| ch[i]).sum();
        }

        ProcessStatus::Ready
    }

    fn block_size(&self) -> BlockRequirements {
        BlockRequirements {
            input_size: BlockSize::Flexible,
            output_size: BlockSize::Fixed(1),
        }
    }
}

impl Default for SummingMixer {
    fn default() -> Self {
        Self {
            out_frame: vec![0.0],
        }
    }
}
