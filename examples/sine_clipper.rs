use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::SampleFormat;
use oryzae::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or("No output device available")?;

    let config = device.default_output_config()?;

    // TODO: bail if setting fails
    let _ = SYSTEM_SAMPLE_RATE.set(config.sample_rate().0 as f32);

    let sine_osc_1 = FastSineOsc::new(120.0);
    let sine_osc_2 = FastSineOsc::new(2.0);
    let sine_osc_3 = FastSineOsc::new(4.0 * (1.0 / 3.0));
    let network1 = Series::new(vec![
        Parallel::new(vec![sine_osc_1, sine_osc_2, sine_osc_3]),
        SummingMixer::new(),
        Gain::new(5.9),
        SoftClipper::new(),
        Gain::new(0.9),
    ]);

    let sine_osc_1 = FastSineOsc::new(480.0);
    let sine_osc_2 = FastSineOsc::new(5.0);
    let sine_osc_3 = FastSineOsc::new(2.0 * (1.0 / 3.0));
    let network2 = Series::new(vec![
        Parallel::new(vec![sine_osc_1, sine_osc_2, sine_osc_3]),
        SummingMixer::new(),
        Gain::new(5.9),
        SoftClipper::new(),
        Gain::new(0.9),
    ]);
    let network = Series::new(vec![Parallel::new(vec![network1, network2])]);

    let stream = match config.sample_format() {
        SampleFormat::F32 => build_stream::<f32>(&device, &config.into(), network)?,
        SampleFormat::I16 => build_stream::<i16>(&device, &config.into(), network)?,
        SampleFormat::U16 => build_stream::<u16>(&device, &config.into(), network)?,
        _ => return Err("Unsupported sample format".into()),
    };

    stream.play()?;

    println!("Playing. Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(())
}