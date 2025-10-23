# Oryzae

An audio processing playground for experimenting with modular network architectures.

## Overview

This repo explores different approaches to building audio processing networks. 

Currently I'm experimenting with using a trait-based design all audio components implement a common `Network` trait, allowing easy composition of oscillators, effects, and routing. This network design is heavily inspired by (Marsayas)[https://github.com/marsyas/marsyas].

Ideas I want to play with going forward include matrix based input -> output mapping. And the interplay between control rate signals and audio rate signals. Obviously adding more modules and utilities.

## Running

```bash
cargo run
```

This starts the example audio playback with the current network configuration. Press Enter to stop.

## Current Network Examples

### Parallel Processing
```rust
Parallel::new(vec![sine_osc_440, sine_osc_442])
```
Two sine oscillators (80Hz and 84Hz) running in parallel, each outputting to separate channels.

### Signal Routing
```rust
Series::new(vec![
    Box::new(Parallel::new(vec![sine_osc_440, sine_osc_442])),
    Box::new(SummingMixer::new()),
    Box::new(SoftCliper::new()),
])
```
A complete signal chain: parallel oscillators → summing mixer → soft clipper. This demonstrates how networks can be nested and chained.

### Components

- **SineOsc**: Basic sine wave oscillator with configurable frequency
- **SummingMixer**: Combines multiple input channels into a single output
- **SoftCliper**: Applies soft clipping distortion using a cubic polynomial
- **Parallel**: Routes input to multiple sub-networks simultaneously
- **Series**: Chains networks sequentially

## Architecture

The `Network` trait provides a unified interface:
```rust
fn get_frame(&mut self, in_frame: &[f32]) -> &[f32]
```

This allows any component to process audio frames and be composed with others, enabling rapid experimentation with different signal routing and processing ideas.
