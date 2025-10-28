use crate::network::Network;

/// Parallel routing component
/// 
/// Processes multiple networks in parallel, with each network receiving the same input.
/// The output is a multi-channel frame where each channel corresponds to one sub-network's output.
pub struct Parallel {
    sub_networks: Vec<Box<dyn Network + Send>>,
    out: Vec<f32>,
}

impl Parallel {
    /// Create a new parallel routing component
    /// 
    /// # Arguments
    /// * `networks` - Vector of networks to process in parallel
    /// 
    /// # Returns
    /// * Boxed Parallel instance
    pub fn new(networks: Vec<Box<dyn Network + Send>>) -> Box<Self> {
        let out_len = networks.len();
        Box::new(Self {
            sub_networks: networks,
            out: vec![0.0; out_len],
        })
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