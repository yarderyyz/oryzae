//! Oryzae - A modular audio synthesis library
//!
//! This library provides a network-based architecture for real-time audio processing
//! using the `Network` trait. All audio processing components implement this trait,
//! allowing for flexible composition of oscillators, mixers, effects, and routing.

pub mod audio;
pub mod effects;
pub mod mixers;
pub mod network;
pub mod oscillators;
pub mod routing;
pub mod spectrum;

// Re-export core types for convenience
pub use network::{
    Network, NetworkComplexComplex, NetworkComplexReal, NetworkRealComplex, NetworkRealReal,
};

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::audio::*;
    pub use crate::effects::*;
    pub use crate::mixers::*;
    pub use crate::network::*;
    pub use crate::oscillators::*;
    pub use crate::routing::*;
}
