use crate::network::{
    BlockRequirements, BlockSize, NetworkRealReal, ProcessStatus, RealBuffer, RealBufferMut,
};

/// Parallel routing component
///
/// Processes multiple networks in parallel, with each network receiving the same input.
/// The output is a multi-channel frame where each channel corresponds to one sub-network's output.
pub struct Parallel {
    sub_networks: Vec<Box<dyn NetworkRealReal<f64> + Send>>,
    output_buffers: Vec<Vec<f64>>,
}

impl Parallel {
    /// Create a new parallel routing component
    ///
    /// # Arguments
    /// * `networks` - Vector of networks to process in parallel
    ///
    /// # Returns
    /// * Boxed Parallel instance
    pub fn new(networks: Vec<Box<dyn NetworkRealReal<f64> + Send>>) -> Box<Self> {
        let num_networks = networks.len();
        Box::new(Self {
            sub_networks: networks,
            output_buffers: vec![vec![0.0; 1024]; num_networks],
        })
    }
}

impl NetworkRealReal<f64> for Parallel {
    fn process(&mut self, input: RealBuffer<f64>, output: RealBufferMut<f64>) -> ProcessStatus {
        if self.sub_networks.is_empty() {
            return ProcessStatus::Ready;
        }

        // Process each network in parallel (same input to all)
        for (i, network) in self.sub_networks.iter_mut().enumerate() {
            let frame_len = input[0].len();
            let (buffer, _) = self.output_buffers[i].split_at_mut(frame_len);
            let mut temp_output_refs = [buffer];

            let status = network.process(input, &mut temp_output_refs);
            if status != ProcessStatus::Ready {
                return status;
            }
        }

        // Copy results to output - each sub-network's first channel becomes an output channel
        for (out_ch, network_buffer) in output.iter_mut().zip(self.output_buffers.iter()) {
            let copy_len = out_ch.len().min(input[0].len());
            if copy_len > network_buffer.len() {
                panic!("Network didn't process enough data");
            }
            out_ch[..copy_len].copy_from_slice(&network_buffer[..copy_len]);
        }

        ProcessStatus::Ready
    }

    fn block_size(&self) -> BlockRequirements {
        BlockRequirements {
            input_size: BlockSize::Fixed(1),
            output_size: BlockSize::Fixed(1),
        }
    }
}
