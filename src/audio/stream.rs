use crate::network::{NetworkRealReal, ProcessStatus};
use cpal::traits::DeviceTrait;
use cpal::{FromSample, Sample};

/// Build an audio output stream for the given device and network
///
/// Creates a real-time audio stream that continuously calls the network's process method
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
    mut network: Box<dyn NetworkRealReal<f64> + Send>,
) -> Result<cpal::Stream, cpal::BuildStreamError>
where
    T: Sample + cpal::SizedSample + FromSample<f64>,
{
    let channels = config.channels as usize;

    // Pre-allocate buffers outside the audio thread
    let mut left_buffer = vec![0.0f64; 1024];
    let mut right_buffer = vec![0.0f64; 1024];
    let empty_input_buffer = vec![0.0f64; 1];

    let stream = device.build_output_stream(
        config,
        move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
            for frame in output.chunks_mut(channels) {
                // Setup input and output buffer references without allocation
                let input_slice = &empty_input_buffer[..];
                let input_refs = [input_slice];

                let mut output_refs = [&mut left_buffer[..1], &mut right_buffer[..1]];

                let status = network.process(&input_refs, &mut output_refs[..channels.min(2)]);
                if status != ProcessStatus::Ready {
                    // Fill with silence on error
                    left_buffer[0] = 0.0;
                    right_buffer[0] = 0.0;
                }

                // Copy to output frame
                for (channel, out_sample) in frame.iter_mut().enumerate() {
                    let value = if channel == 0 {
                        Sample::from_sample(left_buffer[0])
                    } else if channel == 1 && channels >= 2 {
                        Sample::from_sample(right_buffer[0])
                    } else {
                        Sample::from_sample(0.0f64) // Additional channels get silence
                    };
                    *out_sample = value;
                }
            }
        },
        |err| eprintln!("Stream error: {}", err),
        None,
    )?;

    Ok(stream)
}
