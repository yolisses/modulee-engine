use crate::{declare_get_id, node_trait::NodeTrait, values_by_id::ValuesById};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Extras {
    value: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct ConstantNode {
    id: usize,
    extras: Extras,
}

declare_get_id! {ConstantNode}

impl NodeTrait for ConstantNode {
    fn process(&mut self, _node_values: &ValuesById) -> f32 {
        self.extras.value
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![]
    }

    fn update(&mut self, new_node: &Self) {
        self.extras = new_node.extras.clone();
    }
}
