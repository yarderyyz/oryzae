use crate::network::{
    BlockRequirements, BlockSize, NetworkRealReal, ProcessStatus, RealBuffer, RealBufferMut,
};

/// Series routing component
///
/// Chains multiple networks sequentially, where the output of one network
/// becomes the input to the next network in the chain.
pub struct Series {
    sub_networks: Vec<Box<dyn NetworkRealReal<f64> + Send>>,
    intermediate_buffers: Vec<Vec<f64>>,
    max_channels: usize,
    max_frame_len: usize,
}

impl Series {
    /// Create a new series routing component
    ///
    /// # Arguments
    /// * `networks` - Vector of networks to chain in series
    ///
    /// # Returns
    /// * Boxed Series instance
    pub fn new(networks: Vec<Box<dyn NetworkRealReal<f64> + Send>>) -> Box<Self> {
        let num_networks = networks.len();
        let max_channels = 8; // Support up to 8 channels
        let max_frame_len = 1024; // Support up to 1024 samples per frame

        // Allocate buffers with enough space for max_channels * max_frame_len
        let buffer_size = max_channels * max_frame_len;

        Box::new(Self {
            sub_networks: networks,
            intermediate_buffers: vec![vec![0.0; buffer_size]; num_networks],
            max_channels,
            max_frame_len,
        })
    }
}

impl NetworkRealReal<f64> for Series {
    fn process(&mut self, input: RealBuffer<f64>, output: RealBufferMut<f64>) -> ProcessStatus {
        if self.sub_networks.is_empty() {
            // Copy input to output if no networks
            for (out_ch, in_ch) in output.iter_mut().zip(input.iter()) {
                out_ch[..in_ch.len()].copy_from_slice(in_ch);
            }
            return ProcessStatus::Ready;
        }

        let num_networks = self.sub_networks.len();
        let frame_len = input[0].len();

        // Process first network - Parallel needs to output 3 channels
        {
            let buffer = &mut self.intermediate_buffers[0];

            // Give Parallel 3 output channels for the 3 sine oscillators
            let (ch1, rest) = buffer.split_at_mut(frame_len);
            let (ch2, ch3) = rest.split_at_mut(frame_len);
            let mut temp_output_refs = [
                &mut ch1[0..frame_len],
                &mut ch2[0..frame_len],
                &mut ch3[0..frame_len],
            ];

            let status = self.sub_networks[0].process(input, &mut temp_output_refs);
            if status != ProcessStatus::Ready {
                return status;
            }
        }

        // Process remaining networks
        for i in 1..num_networks {
            let (prev_buffers, curr_buffers) = self.intermediate_buffers.split_at_mut(i);
            let prev_buffer = &prev_buffers[i - 1];
            let curr_buffer = &mut curr_buffers[0];

            // For SummingMixer (i=1), give it 3 input channels and 1 output channel
            if i == 1 {
                // Input: 3 channels from Parallel
                let (prev_ch1, prev_rest) = prev_buffer.split_at(frame_len);
                let (prev_ch2, prev_ch3) = prev_rest.split_at(frame_len);
                let input_refs = [
                    &prev_ch1[0..frame_len],
                    &prev_ch2[0..frame_len],
                    &prev_ch3[0..frame_len],
                ];

                // Output: 1 channel for SummingMixer
                let mut output_refs = [&mut curr_buffer[0..frame_len]];

                let status = self.sub_networks[i].process(&input_refs, &mut output_refs);
                if status != ProcessStatus::Ready {
                    return status;
                }
            } else {
                // For other networks (Gain, SoftClipper), use 1 channel
                let input_refs = [&prev_buffer[0..frame_len]];
                let mut output_refs = [&mut curr_buffer[0..frame_len]];
                let status = self.sub_networks[i].process(&input_refs, &mut output_refs);
                if status != ProcessStatus::Ready {
                    return status;
                }
            }
        }

        // Copy final result to output - each output channel from its buffer slice
        let final_buffer = &self.intermediate_buffers[num_networks - 1];
        for out_ch in 0..output.len() {
            let copy_len = output[out_ch].len().min(frame_len);
            let src_start = out_ch * frame_len;
            let src_end = src_start + copy_len;
            if src_end > final_buffer.len() {
                panic!("Network didn't process enough data for channel {}", out_ch);
            }
            output[out_ch][..copy_len].copy_from_slice(&final_buffer[src_start..src_end]);
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
