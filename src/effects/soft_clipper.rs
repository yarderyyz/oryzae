use crate::network::Network;

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

impl Network<f64> for SoftClipper {
    fn get_frame(&mut self, in_frame: &[f64]) -> &[f64] {
        if self.out_frame.len() != in_frame.len() {
            self.out_frame.resize(in_frame.len(), 0.0)
        }
        self.out_frame
            .iter_mut()
            .enumerate()
            .for_each(|(channel, sample)| *sample = soft_clip(in_frame[channel]));

        self.out_frame.as_slice()
    }
}

impl Default for SoftClipper {
    fn default() -> Self {
        Self { out_frame: vec![] }
    }
}

