use crate::network::{
    BlockRequirements, BlockSize, NetworkRealReal, ProcessStatus, RealBuffer, RealBufferMut,
};

/// Soft limiting function
#[inline]
fn soft_limit(x: f64) -> f64 {
    x * (27.0 + x * x) / (27.0 + 9.0 * x * x)
}

/// Soft clipping function with smooth limiting
#[inline]
fn soft_clip(x: f64) -> f64 {
    match x {
        x if x < -3.0 => -1.0,
        x if x > 3.0 => 1.0,
        x => soft_limit(x),
    }
}

/// Soft clipping distortion effect
///
/// Applies gentle saturation and limiting to prevent harsh clipping.
/// Uses a smooth mathematical function to gradually compress loud signals.
pub struct SoftClipper {
    out_frame: Vec<f64>,
}

impl SoftClipper {
    /// Create a new soft clipper effect
    ///
    /// # Returns
    /// * Boxed SoftClipper instance
    pub fn new() -> Box<Self> {
        Box::new(Self { out_frame: vec![] })
    }
}

impl NetworkRealReal<f64> for SoftClipper {
    fn process(&mut self, input: RealBuffer<f64>, output: RealBufferMut<f64>) -> ProcessStatus {
        if input.is_empty() || output.is_empty() {
            return ProcessStatus::Ready;
        }

        let channels = input.len().min(output.len());
        for ch in 0..channels {
            let samples = input[ch].len().min(output[ch].len());
            for i in 0..samples {
                output[ch][i] = soft_clip(input[ch][i]);
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

impl Default for SoftClipper {
    fn default() -> Self {
        Self { out_frame: vec![] }
    }
}
