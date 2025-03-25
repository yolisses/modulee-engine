use crate::{declare_get_id, declare_get_input_ids, declare_input_ids, declare_update};
use crate::{node_trait::NodeTrait, values_by_id::ValuesById};
use serde::Deserialize;

declare_input_ids!(phase);

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct SineWaveNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {SineWaveNode}
declare_update! {SineWaveNode}
declare_get_input_ids! {SineWaveNode, phase}

impl NodeTrait for SineWaveNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        let phase = node_values[&self.input_ids.phase];
        (phase * 2.0 * std::f32::consts::PI).sin()
    }
}
