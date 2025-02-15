use crate::{
    node_trait::NodeTrait, set_note_trait::SetNoteTrait, sort::has_id::HasId,
    values_by_id::ValuesById,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct PitchNode {
    id: usize,
    #[serde(skip)]
    pitch: f32,
}

impl NodeTrait for PitchNode {
    fn process(&mut self, _node_values: &ValuesById) -> f32 {
        self.pitch
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

impl SetNoteTrait for PitchNode {
    fn set_note_on(&mut self, pitch: f32) {
        self.pitch = pitch;
    }

    fn set_note_off(&mut self, _pitch: f32) {}
}
