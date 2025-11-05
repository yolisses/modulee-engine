use crate::node_trait::NodeTrait;
use crate::{declare_get_id, declare_get_input_ids_and_its_getter, declare_update};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct MultiplyNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {MultiplyNode}
declare_update! {MultiplyNode}
declare_get_input_ids_and_its_getter! {MultiplyNode, input1, input2}

impl NodeTrait for MultiplyNode {
    fn process(&mut self, node_values: &[f32], _external_node_values: &[f32]) -> f32 {
        let input1 = unsafe { *node_values.get_unchecked(self.input_ids.input1) };
        let input2 = unsafe { *node_values.get_unchecked(self.input_ids.input2) };
        input1 * input2
    }
}
