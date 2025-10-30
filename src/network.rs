use std::sync::OnceLock;

/// Global system sample rate, set once during audio system initialization
pub static SYSTEM_SAMPLE_RATE: OnceLock<f32> = OnceLock::new();

/// Core trait implemented by all audio processing components
///
/// The Network trait provides a unified interface for audio processing nodes.
/// Each implementation processes an input frame and returns an output frame.
pub trait Network {
    /// Process an input frame and return the output frame
    ///
    /// # Arguments
    /// * `in_frame` - Input audio samples (slice of f32 values)
    ///
    /// # Returns
    /// * Reference to output audio samples
    fn get_frame(&mut self, in_frame: &[f32]) -> &[f32];
}

