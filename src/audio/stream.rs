use crate::network::Network;
use cpal::traits::DeviceTrait;
use cpal::{FromSample, Sample};

/// Build an audio output stream for the given device and network
/// 
/// Creates a real-time audio stream that continuously calls the network's get_frame method
/// to generate audio samples. Supports multiple sample formats (F32, I16, U16).
/// 
/// # Arguments
/// * `device` - CPAL audio device to output to
/// * `config` - Stream configuration (sample rate, channels, etc.)
/// * `network` - Audio processing network to generate samples
/// 
/// # Returns
/// * Result containing the audio stream or build error
pub fn build_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    mut network: Box<dyn Network<f64> + Send>,
) -> Result<cpal::Stream, cpal::BuildStreamError>
where
    T: Sample + cpal::SizedSample + FromSample<f64>,
{
    let channels = config.channels as usize;

    let stream = device.build_output_stream(
        config,
        move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
            for frame in output.chunks_mut(channels) {
                // frame == left/right pair for stereo

                let net_frame = network.get_frame(&[]);
                let mut value = Sample::from_sample(0.0f64);
                for (channel, out_sample) in frame.iter_mut().enumerate() {
                    if channel < net_frame.len() {
                        value = Sample::from_sample(net_frame[channel]);
                        *out_sample = value;
                    } else {
                        *out_sample = value;
                    }
                }
            }
        },
        |err| eprintln!("Stream error: {}", err),
        None,
    )?;

    Ok(stream)
}