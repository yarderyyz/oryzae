use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SampleFormat};
use std::f32::consts::PI;
use std::sync::OnceLock;

static SYSTEM_SAMPLE_RATE: OnceLock<f32> = OnceLock::new();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or("No output device available")?;

    let config = device.default_output_config()?;

    // TODO: bail if setting fails
    let _ = SYSTEM_SAMPLE_RATE.set(config.sample_rate().0 as f32);

    let sine_osc_440 = Box::new(SineOsc::new(80.0));
    let sine_osc_442 = Box::new(SineOsc::new(84.0));
    let network = Box::new(Series::new(vec![
        Box::new(Parallel::new(vec![sine_osc_440, sine_osc_442])),
        Box::new(SummingMixer::new()),
        Box::new(SoftCliper::new()),
    ]));

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

fn build_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    mut network: Box<dyn Network + Send>,
) -> Result<cpal::Stream, cpal::BuildStreamError>
where
    T: Sample + cpal::SizedSample + FromSample<f32>,
{
    let channels = config.channels as usize;

    let stream = device.build_output_stream(
        config,
        move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
            for frame in output.chunks_mut(channels) {
                // frame == left/right pair for stereo

                let net_frame = network.get_frame(&[]);
                let mut value = Sample::from_sample(0.0f32);
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

#[inline]
fn soft_limit(x: f32) -> f32 {
    x * (27.0 + x * x) / (27.0 + 9.0 * x * x)
}

#[inline]
fn soft_clip(x: f32) -> f32 {
    match x {
        x if x < -3.0 => -1.0,
        x if x > 3.0 => 1.0,
        x => soft_limit(x),
    }
}

struct SoftCliper {
    out_frame: Vec<f32>,
}

impl SoftCliper {
    fn new() -> Self {
        Self { out_frame: vec![] }
    }
}
impl Network for SoftCliper {
    fn get_frame(&mut self, in_frame: &[f32]) -> &[f32] {
        if self.out_frame.len() != in_frame.len() {
            self.out_frame.resize(in_frame.len(), 0.0)
        }
        self.out_frame
            .iter_mut()
            .enumerate()
            .for_each(|(channel, sample)| *sample = soft_clip(in_frame[channel]));

        self.out_frame.as_slice()
    }
}

pub trait Network {
    fn get_frame(&mut self, in_frame: &[f32]) -> &[f32];
}

struct Parallel {
    sub_networks: Vec<Box<dyn Network + Send>>,
    out: Vec<f32>,
}
impl Parallel {
    fn new(networks: Vec<Box<dyn Network + Send>>) -> Self {
        let out_len = networks.len();
        Self {
            sub_networks: networks,
            out: vec![0.0; out_len],
        }
    }
}
impl Network for Parallel {
    fn get_frame(&mut self, in_frame: &[f32]) -> &[f32] {
        for (index, network) in self.sub_networks.iter_mut().enumerate() {
            // How do we handle sub networks that are multi channel?
            self.out[index] = network.get_frame(in_frame)[0];
        }
        self.out.as_slice()
    }
}

struct Series {
    sub_networks: Vec<Box<dyn Network + Send>>,
    out_frame: Vec<f32>,
}
impl Series {
    fn new(networks: Vec<Box<dyn Network + Send>>) -> Self {
        Self {
            sub_networks: networks,
            out_frame: vec![0.0],
        }
    }
}
impl Network for Series {
    fn get_frame(&mut self, in_frame: &[f32]) -> &[f32] {
        let out_frame = self
            .sub_networks
            .iter_mut()
            .fold(in_frame, |acc_frame, network| network.get_frame(acc_frame));
        if self.out_frame.len() != out_frame.len() {
            self.out_frame.resize(out_frame.len(), 0.0)
        }
        self.out_frame.copy_from_slice(out_frame);
        self.out_frame.as_slice()
    }
}

struct SummingMixer {
    out_frame: Vec<f32>,
}
impl SummingMixer {
    fn new() -> Self {
        Self {
            out_frame: vec![0.0],
        }
    }
}
impl Network for SummingMixer {
    fn get_frame(&mut self, in_frame: &[f32]) -> &[f32] {
        self.out_frame[0] = in_frame.iter().sum();
        self.out_frame.as_slice()
    }
}

struct SineOsc {
    phase: f32,
    phase_increment: f32,
    out: Vec<f32>,
}

impl SineOsc {
    fn new(frequency: f32) -> Self {
        Self {
            phase_increment: frequency * 2.0 * PI / SYSTEM_SAMPLE_RATE.get().unwrap(),
            phase: 0.0f32,
            out: vec![0.0],
        }
    }
}

impl Network for SineOsc {
    fn get_frame(&mut self, _: &[f32]) -> &[f32] {
        let current_phase = self.phase;
        let sample = current_phase.sin();
        let mut new_phase = current_phase + self.phase_increment;
        if new_phase >= 2.0 * PI {
            new_phase -= 2.0 * PI;
        }
        self.phase = new_phase;
        self.out[0] = sample;
        self.out.as_slice()
    }
}
