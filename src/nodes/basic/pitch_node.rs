use crate::{
    declare_empty_get_input_ids, declare_get_id, has_update::HasUpdate, node_trait::NodeTrait,
    set_note_trait::SetNoteTrait,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct PitchNode {
    id: usize,
    #[serde(skip)]
    pitch: f32,
}

declare_get_id! {PitchNode}
declare_empty_get_input_ids! {PitchNode}

impl HasUpdate for PitchNode {
    fn update(&mut self, _new_node: &Self) {}
}

impl NodeTrait for PitchNode {
    fn process(&mut self, _node_values: &Vec<f32>) -> f32 {
        self.pitch
    }
}

impl SetNoteTrait for PitchNode {
    fn set_note_on(&mut self, pitch: f32) {
        self.pitch = pitch;
    }

    fn set_note_off(&mut self, _pitch: f32) {}
}
