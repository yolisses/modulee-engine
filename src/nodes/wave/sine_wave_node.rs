use crate::node_trait::NodeTrait;
use crate::{declare_get_id, declare_get_input_ids_and_its_getter, declare_update};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct SineWaveNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {SineWaveNode}
declare_update! {SineWaveNode}
declare_get_input_ids_and_its_getter! {SineWaveNode, phase}

impl NodeTrait for SineWaveNode {
    fn process(&mut self, node_values: &[f32], _external_node_values: &[f32]) -> f32 {
        let phase = node_values[self.input_ids.phase];
        (phase * 2.0 * std::f32::consts::PI).sin()
    }
}
