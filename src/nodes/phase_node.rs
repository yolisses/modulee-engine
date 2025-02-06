use crate::{
    node_trait::NodeTrait, values_by_id::ValuesById, sample_rate::SAMPLE_RATE, sort::has_id::HasId,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct InputIds {
    frequency: usize,
}

/// Returns the phase value between 0 and 1 given a time and a frequency
#[derive(Debug, Deserialize)]
pub(crate) struct PhaseNode {
    id: usize,
    input_ids: InputIds,
    #[serde(skip)]
    phase: f32,
}

impl NodeTrait for PhaseNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        let frequency = node_values.get(&self.input_ids.frequency).unwrap();

        self.phase += frequency / SAMPLE_RATE;
        self.phase %= 1.;
        self.phase
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![self.input_ids.frequency]
    }
}

impl HasId for PhaseNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
