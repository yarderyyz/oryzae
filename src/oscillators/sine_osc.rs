use crate::network::{Network, SYSTEM_SAMPLE_RATE};
use std::f32::consts::PI;

/// High-quality sine wave oscillator
/// 
/// Generates clean sine waves using the standard sin() function.
/// Suitable for audio-rate synthesis where quality is important.
pub struct SineOsc {
    phase: f32,
    phase_increment: f32,
    out: Vec<f32>,
}

impl SineOsc {
    /// Create a new sine oscillator
    /// 
    /// # Arguments
    /// * `frequency` - Frequency in Hz
    /// 
    /// # Returns
    /// * Boxed SineOsc instance
    pub fn new(frequency: f32) -> Box<Self> {
        Box::new(Self {
            phase_increment: frequency * 2.0 * PI / SYSTEM_SAMPLE_RATE.get().unwrap(),
            phase: 0.0f32,
            out: vec![0.0],
        })
    }
}

impl Network for SineOsc {
    fn get_frame(&mut self, _: &[f32]) -> &[f32] {
        let current_phase = self.phase;
        let sample = current_phase.sin();
        let mut new_phase = current_phase + self.phase_increment;
        if new_phase >= 2.0 * PI {
            new_phase -= 2.0 * PI;
        }
        self.phase = new_phase;
        self.out[0] = sample;
        self.out.as_slice()
    }
}