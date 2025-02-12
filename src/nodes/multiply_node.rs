use crate::{node_trait::NodeTrait, sort::has_id::HasId, values_by_id::ValuesById};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct InputIds {
    input1: usize,
    input2: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct MultiplyNode {
    id: usize,
    input_ids: InputIds,
}

impl NodeTrait for MultiplyNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        let input1 = node_values[&self.input_ids.input1];
        let input2 = node_values[&self.input_ids.input2];
        input1 * input2
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![self.input_ids.input1, self.input_ids.input2]
    }
}

impl HasId for MultiplyNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
