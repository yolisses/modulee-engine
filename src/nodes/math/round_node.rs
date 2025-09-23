use crate::node_trait::NodeTrait;
use crate::{declare_get_id, declare_get_input_ids_and_its_getter, declare_update};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct RoundNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {RoundNode}
declare_update! {RoundNode}
declare_get_input_ids_and_its_getter! {RoundNode, input1}

impl NodeTrait for RoundNode {
    fn process(&mut self, node_values: &[f32]) -> f32 {
        let input1 = node_values[self.input_ids.input1];
        input1.round()
    }
}
