use std::collections::HashMap;

use crate::{group::Group, values_by_id::ValuesById};
use serde::Deserialize;

// TODO check if all these derives make sense to be used here
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct VoiceGroup {
    pitch: f32,
    group: Group,
}

impl VoiceGroup {
    pub(crate) fn new(pitch: f32, group: Group) -> Self {
        Self { pitch, group }
    }

    pub(crate) fn get_pitch(&self) -> f32 {
        return self.pitch;
    }

    // This may be a violation of the responsibility division, but improves
    // performance
    pub(crate) fn update_input_nodes(
        &mut self,
        node_values: &ValuesById,
        input_target_ids: &HashMap<usize, usize>,
    ) {
        self.group.update_input_nodes(node_values, input_target_ids);
    }

    pub fn get_output_value(&self) -> f32 {
        self.group.get_output_value()
    }

    pub fn process(&mut self) {
        self.group.process();
    }

    pub fn set_note_on(&mut self, pitch: f32) {
        self.group.set_note_on(pitch);
    }

    pub fn set_note_off(&mut self, pitch: f32) {
        self.group.set_note_off(pitch);
    }
}
