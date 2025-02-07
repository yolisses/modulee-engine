use crate::{node_trait::NodeTrait, sort::has_id::HasId, values_by_id::ValuesById};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct PitchNode {
    id: usize,
    #[serde(skip)]
    pitch: f32,
}

impl PitchNode {
    pub(crate) fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch;
    }
}

impl NodeTrait for PitchNode {
    fn process(&mut self, node_values: &mut ValuesById) {
        let value = self.pitch;
        node_values.insert(self.id, value);
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![]
    }
}

impl HasId for PitchNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
