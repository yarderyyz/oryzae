use crate::network::{Network, SYSTEM_SAMPLE_RATE};

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

impl Network<f64> for FastSineOsc {
    fn get_frame(&mut self, _: &[f64]) -> &[f64] {
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
        self.out[0] = sample;
        self.out.as_slice()
    }
}