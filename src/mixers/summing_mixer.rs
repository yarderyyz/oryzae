use crate::network::Network;

/// Summing mixer that adds all input channels together
/// 
/// Takes multiple input channels and sums them into a single mono output channel.
/// Useful for combining multiple audio sources.
pub struct SummingMixer {
    out_frame: Vec<f32>,
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

impl Network for SummingMixer {
    fn get_frame(&mut self, in_frame: &[f32]) -> &[f32] {
        self.out_frame[0] = in_frame.iter().sum();
        self.out_frame.as_slice()
    }
}

impl Default for SummingMixer {
    fn default() -> Self {
        Self {
            out_frame: vec![0.0],
        }
    }
}