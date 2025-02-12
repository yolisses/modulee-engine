use crate::{
    group::Group, node_trait::NodeTrait, sort::has_id::HasId, values_by_id::ValuesById,
    voice::Voice,
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Extras {
    input_target_ids: HashMap<usize, usize>,
}

/// Returns the phase value between 0 and 1 given a time and a frequency
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct GroupVoicesNode {
    id: usize,
    extras: Extras,
    #[serde(skip)]
    group: Group,
    #[serde(skip)]
    voices: Vec<Voice>,
}

impl GroupVoicesNode {
    pub fn set_note_on(&mut self, pitch: f32) {
        let group = self.group.clone();
        let mut voice_group = Voice::new(pitch, group);
        voice_group.group.set_note_on(pitch);
        self.voices.push(voice_group);
    }

    pub fn set_note_off(&mut self, pitch: f32) {
        for voice_group in &mut self.voices {
            if voice_group.pitch == pitch {
                voice_group.group.set_note_off(pitch);
            }
        }
    }
}

impl NodeTrait for GroupVoicesNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        let mut sum = 0.;
        for voice in &mut self.voices {
            voice
                .group
                .update_input_nodes(node_values, &self.extras.input_target_ids);
            voice.group.process();
            sum += voice.group.get_output_value()
        }
        sum
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![]
    }
}

impl HasId for GroupVoicesNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
