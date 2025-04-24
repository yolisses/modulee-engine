use crate::node_trait::NodeTrait;
use crate::{declare_get_id, declare_get_input_ids_and_its_getter, declare_update};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct SubtractNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {SubtractNode}
declare_update! {SubtractNode}
declare_get_input_ids_and_its_getter! {SubtractNode, input1, input2}

impl NodeTrait for SubtractNode {
    fn process(&mut self, node_values: &Vec<f32>) -> f32 {
        let input1 = node_values[self.input_ids.input1];
        let input2 = node_values[self.input_ids.input2];
        input1 - input2
    }
}
