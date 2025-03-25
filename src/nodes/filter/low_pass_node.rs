use crate::{
    declare_get_id, filter::low_pass::LowPass, node_trait::NodeTrait, values_by_id::ValuesById,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct InputIds {
    input: usize,
    frequency: usize,
    resonance: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct LowPassNode {
    id: usize,
    input_ids: InputIds,
    #[serde(skip)]
    low_pass: LowPass,
}

declare_get_id! {LowPassNode}

impl NodeTrait for LowPassNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        let input = node_values[&self.input_ids.input];
        let frequency = node_values[&self.input_ids.frequency];
        let resonance = node_values[&self.input_ids.resonance];
        self.low_pass.process(input, frequency, resonance)
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![
            self.input_ids.input,
            self.input_ids.frequency,
            self.input_ids.resonance,
        ]
    }

    fn update(&mut self, new_node: &Self) {
        self.input_ids = new_node.input_ids.clone();
    }
}
