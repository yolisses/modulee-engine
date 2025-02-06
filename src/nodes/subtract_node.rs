use crate::{node_trait::NodeTrait, values_by_id::ValuesById, sort::has_id::HasId};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct InputIds {
    input1: usize,
    input2: usize,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SubtractNode {
    id: usize,
    input_ids: InputIds,
}

impl NodeTrait for SubtractNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        let input1 = node_values.get(&self.input_ids.input1).unwrap();
        let input2 = node_values.get(&self.input_ids.input2).unwrap();
        input1 - input2
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![self.input_ids.input1, self.input_ids.input2]
    }
}

impl HasId for SubtractNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
