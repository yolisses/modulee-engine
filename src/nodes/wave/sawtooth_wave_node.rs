use crate::{declare_get_id, declare_get_input_ids, declare_input_ids, declare_update};
use crate::{node_trait::NodeTrait, values_by_id::ValuesById};
use serde::Deserialize;

declare_input_ids!(phase);

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct SawtoothWaveNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {SawtoothWaveNode}
declare_update! {SawtoothWaveNode}
declare_get_input_ids! {SawtoothWaveNode, phase}

impl NodeTrait for SawtoothWaveNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        let phase = node_values[&self.input_ids.phase];
        2. * phase - 1.
    }
}
