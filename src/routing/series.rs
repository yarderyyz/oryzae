use crate::network::Network;

/// Series routing component
///
/// Chains multiple networks sequentially, where the output of one network
/// becomes the input to the next network in the chain.
pub struct Series {
    sub_networks: Vec<Box<dyn Network<f64> + Send>>,
    out_frame: [f64; 32],
}

impl Series {
    /// Create a new series routing component
    ///
    /// # Arguments
    /// * `networks` - Vector of networks to chain in series
    ///
    /// # Returns
    /// * Boxed Series instance
    pub fn new(networks: Vec<Box<dyn Network<f64> + Send>>) -> Box<Self> {
        Box::new(Self {
            sub_networks: networks,
            out_frame: [0.0; 32],
        })
    }
}

impl Network<f64> for Series {
    fn get_frame(&mut self, in_frame: &[f64]) -> &[f64] {
        let network_result = self
            .sub_networks
            .iter_mut()
            .fold(in_frame, |acc_frame, network| network.get_frame(acc_frame));

        if network_result.len() > 32 {
            panic!("Exceeded max of 32 channels");
        }

        self.out_frame[0..network_result.len()].copy_from_slice(network_result);
        &self.out_frame[0..network_result.len()]
    }
}

