use crate::{
    declare_get_id, declare_get_input_ids_and_its_getter, declare_update,
    default_sample_rate::DEFAULT_SAMPLE_RATE,
    filter::filter_wrapper_with_gain::FilterWrapperWithGain, node_trait::NodeTrait,
    set_sample_rate_trait::SetSampleRateTrait,
};
use biquad::Type;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct PeakNode {
    id: usize,
    input_ids: InputIds,
    #[serde(skip)]
    #[serde(default = "get_default_filter_wrapper")]
    filter_wrapper_with_gain: FilterWrapperWithGain,
}

fn get_default_filter_wrapper() -> FilterWrapperWithGain {
    FilterWrapperWithGain::new(DEFAULT_SAMPLE_RATE)
}

declare_get_id! {PeakNode}
declare_update! {PeakNode}
declare_get_input_ids_and_its_getter! {PeakNode, input, frequency, resonance, gain}

impl NodeTrait for PeakNode {
    fn process(&mut self, node_values: &[f32], _external_node_values: &[f32]) -> f32 {
        let gain = unsafe { *node_values.get_unchecked(self.input_ids.gain) };
        let input = unsafe { *node_values.get_unchecked(self.input_ids.input) };
        let frequency = unsafe { *node_values.get_unchecked(self.input_ids.frequency) };
        let resonance = unsafe { *node_values.get_unchecked(self.input_ids.resonance) };

        let filter_type = Type::PeakingEQ(gain);
        self.filter_wrapper_with_gain
            .process(input, frequency, resonance, filter_type)
    }
}

impl SetSampleRateTrait for PeakNode {
    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.filter_wrapper_with_gain.set_sample_rate(sample_rate);
    }
}
