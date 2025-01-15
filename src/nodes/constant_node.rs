use crate::{node_trait::NodeTrait, node_values::NodeValues, sort::has_id::HasId};
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct Extras {
    pub(crate) value: f32,
}

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct ConstantNode {
    pub(crate) id: usize,
    pub(crate) extras: Extras,
}

impl ConstantNode {
    pub(crate) fn new(id: usize, value: f32) -> Self {
        Self {
            id,
            extras: Extras { value },
        }
    }
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
