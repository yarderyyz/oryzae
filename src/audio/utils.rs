use cpal::traits::HostTrait;

/// Audio utility functions
/// TODO::honestly just call the methods directly lol

/// Get the default audio host
///
/// # Returns
/// * Default CPAL host
pub fn get_default_host() -> cpal::Host {
    cpal::default_host()
}

/// Get the default output device from a host
///
/// # Arguments
/// * `host` - CPAL host to get device from
///
/// # Returns
/// * Result containing default output device or error message
pub fn get_default_output_device(host: &cpal::Host) -> Result<cpal::Device, &'static str> {
    host.default_output_device()
        .ok_or("No output device available")
}

