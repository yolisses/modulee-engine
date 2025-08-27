use crate::{
    declare_get_id, declare_get_input_ids_and_its_getter, declare_update,
    filter::filter_wrapper::FilterWrapper, node_trait::NodeTrait,
    set_sample_rate_trait::SetSampleRateTrait,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct AllPassNode {
    id: usize,
    input_ids: InputIds,
    #[serde(skip)]
    #[serde(default = "get_default_filter_wrapper")]
    filter_wrapper: FilterWrapper,
}

fn get_default_filter_wrapper() -> FilterWrapper {
    FilterWrapper::new(biquad::Type::AllPass, 1.)
}

declare_get_id! {AllPassNode}
declare_update! {AllPassNode}
declare_get_input_ids_and_its_getter! {AllPassNode, input, frequency, resonance}

impl NodeTrait for AllPassNode {
    fn process(&mut self, node_values: &[f32]) -> f32 {
        let input = node_values[self.input_ids.input];
        let frequency = node_values[self.input_ids.frequency];
        let resonance = node_values[self.input_ids.resonance];
        self.filter_wrapper.process(input, frequency, resonance)
    }
}

impl SetSampleRateTrait for AllPassNode {
    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.filter_wrapper.set_sample_rate(sample_rate);
    }
}
