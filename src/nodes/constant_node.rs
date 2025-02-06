use crate::{node_trait::NodeTrait, values_by_id::ValuesById, sort::has_id::HasId};
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
    fn process(&mut self, _node_values: &ValuesById) -> f32 {
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
