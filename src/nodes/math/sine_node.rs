use crate::node_trait::NodeTrait;
use crate::{declare_get_id, declare_get_input_ids_and_its_getter, declare_update};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct SineNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {SineNode}
declare_update! {SineNode}
declare_get_input_ids_and_its_getter! {SineNode, input}

impl NodeTrait for SineNode {
    fn process(&mut self, node_values: &[f32]) -> f32 {
        let input = node_values[self.input_ids.input];
        input.sin()
    }
}
