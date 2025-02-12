use crate::sort::has_id::HasId;
use crate::{node_trait::NodeTrait, values_by_id::ValuesById};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct InputIds {
    input1: usize,
    input2: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct AddNode {
    id: usize,
    input_ids: InputIds,
}

impl NodeTrait for AddNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        let input1 = node_values[&self.input_ids.input1];
        let input2 = node_values[&self.input_ids.input2];
        input1 + input2
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![self.input_ids.input1, self.input_ids.input2]
    }
}

impl HasId for AddNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
