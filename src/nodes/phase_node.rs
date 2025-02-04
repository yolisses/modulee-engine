use crate::{node_trait::NodeTrait, node_values::NodeValues, sort::has_id::HasId};
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct InputIds {
    time: usize,
    frequency: usize,
}

/// Returns the phase value between 0 and 1 given a time and a frequency
#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct PhaseNode {
    id: usize,
    input_ids: InputIds,
}

impl NodeTrait for PhaseNode {
    fn process(&mut self, node_values: &NodeValues) -> f32 {
        let time = node_values.get(&self.input_ids.time).unwrap();
        let frequency = node_values.get(&self.input_ids.frequency).unwrap();

        let product = time * frequency;
        product - product.floor()
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![self.input_ids.time, self.input_ids.frequency]
    }
}

impl HasId for PhaseNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
