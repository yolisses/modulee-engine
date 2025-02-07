use crate::{node_trait::NodeTrait, sort::has_id::HasId, values_by_id::ValuesById};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct InputIds {
    phase: usize,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TriangleWaveNode {
    id: usize,
    input_ids: InputIds,
}

impl NodeTrait for TriangleWaveNode {
    fn process(&mut self, node_values: &mut ValuesById) {
        let phase = node_values.get(&self.input_ids.phase).unwrap();
        let t = phase % 1.0;
        let value = if t < 0.5 {
            4.0 * t - 1.0
        } else {
            3.0 - 4.0 * t
        };
        node_values.insert(self.id, value);
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![self.input_ids.phase]
    }
}

impl HasId for TriangleWaveNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
