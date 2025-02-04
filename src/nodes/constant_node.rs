use crate::{node_trait::NodeTrait, node_values::NodeValues, sort::has_id::HasId};
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct Extras {
    value: f32,
}

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct ConstantNode {
    id: usize,
    extras: Extras,
}

impl NodeTrait for ConstantNode {
    fn process(&mut self, _node_values: &NodeValues) -> f32 {
        self.extras.value
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![]
    }
}

impl HasId for ConstantNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
