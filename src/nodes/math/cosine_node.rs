use crate::node_trait::NodeTrait;
use crate::{declare_get_id, declare_get_input_ids_and_its_getter, declare_update};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct CosineNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {CosineNode}
declare_update! {CosineNode}
declare_get_input_ids_and_its_getter! {CosineNode, input}

impl NodeTrait for CosineNode {
    fn process(&mut self, node_values: &[f32], _external_node_values: &[f32]) -> f32 {
        let input = unsafe { *node_values.get_unchecked(self.input_ids.input) };
        input.cos()
    }
}
