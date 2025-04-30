use crate::node_trait::NodeTrait;
use crate::{declare_get_id, declare_get_input_ids_and_its_getter, declare_update};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct GreaterNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {GreaterNode}
declare_update! {GreaterNode}
declare_get_input_ids_and_its_getter! {GreaterNode, input1, input2}

impl NodeTrait for GreaterNode {
    fn process(&mut self, node_values: &[f32]) -> f32 {
        let input1 = node_values[self.input_ids.input1];
        let input2 = node_values[self.input_ids.input2];
        if input1 > input2 {
            1.
        } else {
            0.
        }
    }
}
