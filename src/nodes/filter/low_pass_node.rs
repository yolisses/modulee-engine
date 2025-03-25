use crate::{
    declare_get_id, declare_get_input_ids_and_its_getter, declare_update,
    filter::low_pass::LowPass, node_trait::NodeTrait, values_by_id::ValuesById,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct LowPassNode {
    id: usize,
    input_ids: InputIds,
    #[serde(skip)]
    low_pass: LowPass,
}

declare_get_id! {LowPassNode}
declare_update! {LowPassNode}
declare_get_input_ids_and_its_getter! {LowPassNode, input, frequency, resonance}

impl NodeTrait for LowPassNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        let input = node_values[&self.input_ids.input];
        let frequency = node_values[&self.input_ids.frequency];
        let resonance = node_values[&self.input_ids.resonance];
        self.low_pass.process(input, frequency, resonance)
    }
}
