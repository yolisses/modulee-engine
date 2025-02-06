use crate::{node_trait::NodeTrait, values_by_id::ValuesById, sort::has_id::HasId};
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct InputIds {
    pitch: usize,
}

/// Receives a pitch in the MIDI format and returns the frequency of the pitch
#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct FrequencyNode {
    id: usize,
    input_ids: InputIds,
}

impl NodeTrait for FrequencyNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        let pitch = node_values.get(&self.input_ids.pitch).unwrap();
        440.0 * 2.0_f32.powf((pitch - 69.0) / 12.0)
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![self.input_ids.pitch]
    }
}

impl HasId for FrequencyNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
