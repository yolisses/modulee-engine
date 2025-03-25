use crate::{
    declare_empty_get_input_ids, declare_get_id, has_update::HasUpdate, node_trait::NodeTrait,
    values_by_id::ValuesById,
};
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
declare_empty_get_input_ids! {ConstantNode}

impl HasUpdate for ConstantNode {
    fn update(&mut self, new_node: &Self) {
        self.extras = new_node.extras.clone();
    }
}

impl NodeTrait for ConstantNode {
    fn process(&mut self, _node_values: &ValuesById) -> f32 {
        self.extras.value
    }
}
