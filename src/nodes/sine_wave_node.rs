use crate::{node_trait::NodeTrait, sort::has_id::HasId, values_by_id::ValuesById};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct InputIds {
    phase: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct SineWaveNode {
    id: usize,
    input_ids: InputIds,
}

impl NodeTrait for SineWaveNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        let phase = node_values[&self.input_ids.phase];
        (phase * 2.0 * std::f32::consts::PI).sin()
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![self.input_ids.phase]
    }

    fn update(&mut self, new_node: &Self) {
        self.input_ids = new_node.input_ids.clone();
    }
}

impl HasId for SineWaveNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
