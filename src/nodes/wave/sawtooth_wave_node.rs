use crate::node_trait::NodeTrait;
use crate::{declare_get_id, declare_get_input_ids_and_its_getter, declare_update};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct SawtoothWaveNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {SawtoothWaveNode}
declare_update! {SawtoothWaveNode}
declare_get_input_ids_and_its_getter! {SawtoothWaveNode, phase}

impl NodeTrait for SawtoothWaveNode {
    fn process(&mut self, node_values: &[f32]) -> f32 {
        let phase = node_values[self.input_ids.phase];
        2. * phase - 1.
    }
}
