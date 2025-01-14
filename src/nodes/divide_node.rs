use crate::{node_trait::NodeTrait, node_values::NodeValues};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct InputIds {
    pub(crate) input1: usize,
    pub(crate) input2: usize,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DivideNode {
    pub(crate) id: usize,
    pub(crate) input_ids: InputIds,
}

impl NodeTrait for DivideNode {
    fn process(&mut self, node_values: &NodeValues) -> f32 {
        let input1 = node_values.get(&self.input_ids.input1).unwrap();
        let input2 = node_values.get(&self.input_ids.input2).unwrap();
        input1 / input2
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![self.input_ids.input1, self.input_ids.input2]
    }
}
