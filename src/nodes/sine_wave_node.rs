use crate::{node_trait::NodeTrait, sort::has_id::HasId, values_by_id::ValuesById};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct InputIds {
    phase: usize,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SineWaveNode {
    id: usize,
    input_ids: InputIds,
}

impl NodeTrait for SineWaveNode {
    fn process(&mut self, node_values: &mut ValuesById) {
        let phase = node_values.get(&self.input_ids.phase).unwrap();
        let value = (phase * 2.0 * std::f32::consts::PI).sin();
        node_values.insert(self.id, value);
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![self.input_ids.phase]
    }
}

impl HasId for SineWaveNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
