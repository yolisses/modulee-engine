use crate::{declare_get_id, declare_get_input_ids, declare_input_ids, declare_update};
use crate::{node_trait::NodeTrait, values_by_id::ValuesById};
use serde::Deserialize;

declare_input_ids!(phase);

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct TriangleWaveNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {TriangleWaveNode}
declare_update! {TriangleWaveNode}
declare_get_input_ids! {TriangleWaveNode, phase}

impl NodeTrait for TriangleWaveNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        let phase = node_values[&self.input_ids.phase];
        let t = phase % 1.0;
        if t < 0.5 {
            4.0 * t - 1.0
        } else {
            3.0 - 4.0 * t
        }
    }
}
