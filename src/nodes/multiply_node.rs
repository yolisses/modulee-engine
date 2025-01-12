use crate::{node_trait::NodeTrait, node_values::NodeValues};

#[derive(Debug)]
pub(crate) struct InputIds {
    pub(crate) input1: usize,
    pub(crate) input2: usize,
}

#[derive(Debug)]
pub(crate) struct MultiplyNode {
    pub(crate) id: usize,
    pub(crate) input_ids: InputIds,
}

impl NodeTrait for MultiplyNode {
    fn process(&mut self, node_values: &NodeValues) -> f64 {
        let input1 = node_values.get(&self.input_ids.input1).unwrap();
        let input2 = node_values.get(&self.input_ids.input2).unwrap();
        input1 * input2
    }

    fn get_id(&self) -> usize {
        self.id
    }
}
