use crate::{
    declare_empty_update, declare_get_id, declare_get_input_ids, node_trait::NodeTrait,
    values_by_id::ValuesById,
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
declare_get_input_ids! {InputNode,}

impl InputNode {
    pub(crate) fn set_value(&mut self, value: f32) {
        self.value = value;
    }
}

impl NodeTrait for InputNode {
    fn process(&mut self, _node_values: &ValuesById) -> f32 {
        self.value
    }
}
