use crate::network::Network;

/// Series routing component
/// 
/// Chains multiple networks sequentially, where the output of one network
/// becomes the input to the next network in the chain.
pub struct Series {
    sub_networks: Vec<Box<dyn Network + Send>>,
    out_frame: Vec<f32>,
}

impl Series {
    /// Create a new series routing component
    /// 
    /// # Arguments
    /// * `networks` - Vector of networks to chain in series
    /// 
    /// # Returns
    /// * Boxed Series instance
    pub fn new(networks: Vec<Box<dyn Network + Send>>) -> Box<Self> {
        Box::new(Self {
            sub_networks: networks,
            out_frame: vec![0.0],
        })
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