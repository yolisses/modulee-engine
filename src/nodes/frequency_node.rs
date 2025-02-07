use crate::{node_trait::NodeTrait, sort::has_id::HasId, values_by_id::ValuesById};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct InputIds {
    pitch: usize,
}

/// Receives a pitch in the MIDI format and returns the frequency of the pitch
#[derive(Debug, Deserialize)]
pub(crate) struct FrequencyNode {
    id: usize,
    input_ids: InputIds,
}

impl NodeTrait for FrequencyNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        let pitch = node_values[&self.input_ids.pitch];
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
