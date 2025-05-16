use crate::{
    declare_get_id, declare_get_input_ids_and_its_getter, declare_update,
    filter::filter_wrapper::FilterWrapper, node_trait::NodeTrait,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct LowPassNode {
    id: usize,
    input_ids: InputIds,
    #[serde(skip)]
    #[serde(default = "get_default_filter_wrapper")]
    filter_wrapper: FilterWrapper,
}

fn get_default_filter_wrapper() -> FilterWrapper {
    FilterWrapper::new(biquad::Type::LowPass)
}

declare_get_id! {LowPassNode}
declare_update! {LowPassNode}
declare_get_input_ids_and_its_getter! {LowPassNode, input, frequency, resonance}

impl NodeTrait for LowPassNode {
    fn process(&mut self, node_values: &[f32]) -> f32 {
        let input = node_values[self.input_ids.input];
        let frequency = node_values[self.input_ids.frequency];
        let resonance = node_values[self.input_ids.resonance];
        self.filter_wrapper.process(input, frequency, resonance)
    }
}
