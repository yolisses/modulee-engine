use crate::sort::has_id::HasId;
use crate::{node_trait::NodeTrait, values_by_id::ValuesById};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct InputIds {
    input1: usize,
    input2: usize,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AddNode {
    id: usize,
    input_ids: InputIds,
}

impl NodeTrait for AddNode {
    fn process(&mut self, node_values: &mut ValuesById) {
        let input1 = node_values.get(&self.input_ids.input1).unwrap();
        let input2 = node_values.get(&self.input_ids.input2).unwrap();
        let value = input1 + input2;
        node_values.insert(self.id, value);
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
