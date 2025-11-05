use crate::{
    declare_get_id, declare_get_input_ids_and_its_getter, declare_update,
    default_sample_rate::DEFAULT_SAMPLE_RATE, filter::filter_wrapper::FilterWrapper,
    node_trait::NodeTrait, set_sample_rate_trait::SetSampleRateTrait,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct HighPassNode {
    id: usize,
    input_ids: InputIds,
    #[serde(skip)]
    #[serde(default = "get_default_filter_wrapper")]
    filter_wrapper: FilterWrapper,
}

fn get_default_filter_wrapper() -> FilterWrapper {
    FilterWrapper::new(biquad::Type::HighPass, DEFAULT_SAMPLE_RATE)
}

declare_get_id! {HighPassNode}
declare_update! {HighPassNode}
declare_get_input_ids_and_its_getter! {HighPassNode, input, frequency, resonance}

impl NodeTrait for HighPassNode {
    fn process(&mut self, node_values: &[f32], _external_node_values: &[f32]) -> f32 {
        let input = unsafe { *node_values.get_unchecked(self.input_ids.input) };
        let frequency = unsafe { *node_values.get_unchecked(self.input_ids.frequency) };
        let resonance = unsafe { *node_values.get_unchecked(self.input_ids.resonance) };
        self.filter_wrapper.process(input, frequency, resonance)
    }
}

impl SetSampleRateTrait for HighPassNode {
    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.filter_wrapper.set_sample_rate(sample_rate);
    }
}
