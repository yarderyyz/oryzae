use num::Complex;
use std::sync::OnceLock;

use num::traits::float;

/// Global system sample rate, set once during audio system initialization
pub static SYSTEM_SAMPLE_RATE: OnceLock<f64> = OnceLock::new();

pub type RealBuffer<'a, T: float::Float> = &'a [&'a [T]];
pub type ComplexBuffer<'a, T: float::Float> = &'a [&'a [Complex<T>]];
pub type RealBufferMut<'a, T: float::Float> = &'a mut [&'a mut [T]];
pub type ComplexBufferMut<'a, T: float::Float> = &'a mut [&'a mut [Complex<T>]];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessStatus {
    Ready,
    NeedMoreInput { samples_needed: usize },
    PartialOutput { frames_written: usize },
}
#[derive(Debug, Clone, Copy)]
pub struct BlockRequirements {
    pub input_size: BlockSize,
    pub output_size: BlockSize,
}

#[derive(Debug, Clone, Copy)]
pub enum BlockSize {
    Flexible,
    Fixed(usize),
    Multiple(usize),
}

/// Core trait implemented by all audio processing components
///
/// The Network trait provides a unified interface for audio processing nodes.
/// Each implementation processes an input frame and returns an output frame.
pub trait NetworkRealReal<T: float::Float> {
    fn process(&mut self, input: RealBuffer<T>, output: RealBufferMut<T>) -> ProcessStatus;
    fn block_size(&self) -> BlockRequirements;
}

pub trait NetworkRealComplex<T: float::Float> {
    fn process(&mut self, input: RealBuffer<T>, output: ComplexBufferMut<T>) -> ProcessStatus;
    fn block_size(&self) -> BlockRequirements;
}

pub trait NetworkComplexComplex<T: float::Float> {
    fn process(&mut self, input: ComplexBuffer<T>, output: ComplexBufferMut<T>) -> ProcessStatus;
    fn block_size(&self) -> BlockRequirements;
}

pub trait NetworkComplexReal<T: float::Float> {
    fn process(&mut self, input: ComplexBuffer<T>, output: RealBufferMut<T>) -> ProcessStatus;
    fn block_size(&self) -> BlockRequirements;
}

pub enum Network<T: float::Float> {
    RealReal(Box<dyn NetworkRealReal<T> + Send>),
    RealComplex(Box<dyn NetworkRealComplex<T> + Send>),
    ComplexComplex(Box<dyn NetworkComplexComplex<T> + Send>),
    ComplexReal(Box<dyn NetworkComplexReal<T> + Send>),
}
