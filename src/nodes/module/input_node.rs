use crate::{
    declare_empty_get_input_ids, declare_empty_update, declare_get_id, node_trait::NodeTrait,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct InputNode {
    id: usize,
    #[serde(skip)]
    value: f32,
}

declare_get_id! {InputNode}
declare_empty_update! {InputNode}
declare_empty_get_input_ids! {InputNode}

impl InputNode {
    pub(crate) fn set_value(&mut self, value: f32) {
        self.value = value;
    }
}

impl NodeTrait for InputNode {
    fn process(&mut self, _node_values: &[f32]) -> f32 {
        self.value
    }
}
