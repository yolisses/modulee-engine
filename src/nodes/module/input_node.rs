use crate::{
    declare_empty_get_input_ids, declare_empty_update, declare_get_id, node_trait::NodeTrait,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct InputNode {
    id: usize,
    #[serde(skip)]
    target: usize,
}

declare_get_id! {InputNode}
declare_empty_update! {InputNode}
declare_empty_get_input_ids! {InputNode}

impl InputNode {
    pub(crate) fn set_target(&mut self, target: usize) {
        self.target = target;
    }
}

impl NodeTrait for InputNode {
    fn process(&mut self, _node_values: &[f32], external_node_values: &[f32]) -> f32 {
        external_node_values[self.target]
    }
}
