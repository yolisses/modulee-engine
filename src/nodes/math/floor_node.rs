use crate::node_trait::NodeTrait;
use crate::{declare_get_id, declare_get_input_ids_and_its_getter, declare_update};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct FloorNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {FloorNode}
declare_update! {FloorNode}
declare_get_input_ids_and_its_getter! {FloorNode, input}

impl NodeTrait for FloorNode {
    fn process(&mut self, node_values: &[f32]) -> f32 {
        let input = node_values[self.input_ids.input];
        input.floor()
    }
}
