use crate::sort::has_id::HasId;
use crate::{node_trait::NodeTrait, node_values::NodeValues};
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct InputIds {
    input1: usize,
    input2: usize,
}

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct AddNode {
    id: usize,
    input_ids: InputIds,
}

impl AddNode {
    pub(crate) fn new(id: usize, input1: usize, input2: usize) -> Self {
        Self {
            id,
            input_ids: InputIds { input1, input2 },
        }
    }
}

impl NodeTrait for AddNode {
    fn process(&mut self, node_values: &NodeValues) -> f32 {
        let input1 = node_values.get(&self.input_ids.input1).unwrap();
        let input2 = node_values.get(&self.input_ids.input2).unwrap();
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
