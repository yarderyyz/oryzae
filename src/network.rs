use std::sync::OnceLock;

use num::traits::float;

/// Global system sample rate, set once during audio system initialization
pub static SYSTEM_SAMPLE_RATE: OnceLock<f64> = OnceLock::new();

/// Core trait implemented by all audio processing components
///
/// The Network trait provides a unified interface for audio processing nodes.
/// Each implementation processes an input frame and returns an output frame.
pub trait Network<T: float::Float> {
    /// Process an input frame and return the output frame
    ///
    /// # Arguments
    /// * `in_frame` - Input audio samples (slice of float values)
    ///
    /// # Returns
    /// * Reference to output audio samples
    fn get_frame(&mut self, in_frame: &[T]) -> &[T];
}
