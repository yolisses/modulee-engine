use crate::node_trait::NodeTrait;
use crate::{declare_get_id, declare_get_input_ids_and_its_getter, declare_update};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct FrequencyNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {FrequencyNode}
declare_update! {FrequencyNode}
declare_get_input_ids_and_its_getter! {FrequencyNode, pitch}

impl NodeTrait for FrequencyNode {
    fn process(&mut self, node_values: &[f32], _external_node_values: &[f32]) -> f32 {
        let pitch = node_values[self.input_ids.pitch];
        440.0 * 2.0_f32.powf((pitch - 69.0) / 12.0)
    }
}
