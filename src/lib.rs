//! Oryzae - A modular audio synthesis library
//!
//! This library provides a network-based architecture for real-time audio processing
//! using the `Network` trait. All audio processing components implement this trait,
//! allowing for flexible composition of oscillators, mixers, effects, and routing.

pub mod network;
pub mod oscillators;
pub mod mixers;
pub mod effects;
pub mod routing;
pub mod audio;

// Re-export core types for convenience
pub use network::Network;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::network::*;
    pub use crate::oscillators::*;
    pub use crate::mixers::*;
    pub use crate::effects::*;
    pub use crate::routing::*;
    pub use crate::audio::*;
}