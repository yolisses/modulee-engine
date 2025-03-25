use crate::{
    declare_get_id, declare_get_input_ids_and_its_getter, declare_update,
    filter::filter_wrapper_with_gain::FilterWrapperWithGain, node_trait::NodeTrait,
    values_by_id::ValuesById,
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
    FilterWrapperWithGain::new()
}

declare_get_id! {PeakNode}
declare_update! {PeakNode}
declare_get_input_ids_and_its_getter! {PeakNode, input, frequency, resonance, gain}

impl NodeTrait for PeakNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        let gain = node_values[&self.input_ids.gain];
        let input = node_values[&self.input_ids.input];
        let frequency = node_values[&self.input_ids.frequency];
        let resonance = node_values[&self.input_ids.resonance];

        let filter_type = Type::PeakingEQ(gain);
        self.filter_wrapper_with_gain
            .process(input, frequency, resonance, filter_type)
    }
}
