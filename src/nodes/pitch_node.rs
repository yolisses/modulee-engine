use crate::{node_trait::NodeTrait, node_values::NodeValues, sort::has_id::HasId};
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
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
    fn process(&mut self, _node_values: &NodeValues) -> f32 {
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
