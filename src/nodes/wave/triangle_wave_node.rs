use crate::node_trait::NodeTrait;
use crate::{declare_get_id, declare_get_input_ids_and_its_getter, declare_update};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct TriangleWaveNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {TriangleWaveNode}
declare_update! {TriangleWaveNode}
declare_get_input_ids_and_its_getter! {TriangleWaveNode, phase}

impl NodeTrait for TriangleWaveNode {
    fn process(&mut self, node_values: &[f32], _external_node_values: &[f32]) -> f32 {
        let phase = unsafe { *node_values.get_unchecked(self.input_ids.phase) };
        let t = phase % 1.0;
        if t < 0.5 {
            4.0 * t - 1.0
        } else {
            3.0 - 4.0 * t
        }
    }
}
