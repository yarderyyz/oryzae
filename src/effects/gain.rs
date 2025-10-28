use crate::network::Network;

/// Gain/volume control effect
/// 
/// Multiplies all input samples by a gain factor. Can be used for
/// volume control, attenuation, or amplification.
pub struct Gain {
    gain: f32,
    out_frame: Vec<f32>,
}

impl Gain {
    /// Create a new gain effect
    /// 
    /// # Arguments
    /// * `gain` - Gain multiplier (1.0 = unity gain, 0.5 = half volume, 2.0 = double volume)
    /// 
    /// # Returns
    /// * Boxed Gain instance
    pub fn new(gain: f32) -> Box<Self> {
        Box::new(Self {
            gain,
            out_frame: vec![],
        })
    }

    /// Set the gain value
    /// 
    /// # Arguments
    /// * `gain` - New gain multiplier
    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain;
    }

    /// Get the current gain value
    /// 
    /// # Returns
    /// * Current gain multiplier
    pub fn get_gain(&self) -> f32 {
        self.gain
    }
}

impl Network for Gain {
    fn get_frame(&mut self, in_frame: &[f32]) -> &[f32] {
        if in_frame.len() != self.out_frame.len() {
            self.out_frame.resize(in_frame.len(), 0.0)
        }
        self.out_frame.copy_from_slice(in_frame);
        self.out_frame.iter_mut().for_each(|x| *x *= self.gain);
        self.out_frame.as_slice()
    }
}