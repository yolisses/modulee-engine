use crate::{node_trait::NodeTrait, node_values::NodeValues, sort::has_id::HasId};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct InputIds {
    pub(crate) phase: usize,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SineWaveNode {
    pub(crate) id: usize,
    pub(crate) input_ids: InputIds,
}

impl SineWaveNode {
    pub(crate) fn new(id: usize, phase: usize) -> Self {
        Self {
            id,
            input_ids: InputIds { phase },
        }
    }
}

impl NodeTrait for SineWaveNode {
    fn process(&mut self, node_values: &NodeValues) -> f32 {
        let phase = node_values.get(&self.input_ids.phase).unwrap();
        (phase * 2.0 * std::f32::consts::PI).sin()
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
