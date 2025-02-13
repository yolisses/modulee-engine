use crate::{
    group::Group, node_trait::NodeTrait, sort::has_id::HasId, values_by_id::ValuesById,
    voice::Voice,
};
use serde::Deserialize;
use std::{collections::HashMap, error::Error};

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Extras {
    target_group_id: usize,
    input_target_ids: HashMap<usize, usize>,
}

/// Returns the phase value between 0 and 1 given a time and a frequency
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct GroupVoicesNode {
    id: usize,
    extras: Extras,
    // TODO fix group instantiation. Currently it only creates an empty group.
    // It should clone a group from graph.
    #[serde(skip)]
    voices: Vec<Voice>,
    #[serde(skip)]
    group: Option<Group>,
}

impl GroupVoicesNode {
    pub(crate) fn update_groups(
        &mut self,
        new_groups: &HashMap<usize, Group>,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(new_group) = new_groups.get(&self.extras.target_group_id) {
            // If the group already exists, updates it
            if let Some(group) = &mut self.group {
                group.update(new_group)?;
            } else {
                // If the group don't exist, saves it
                self.group = Some(new_group.clone());
            }
        } else {
            // If there's no new group version, deletes the group
            self.group = None;
        }
        Ok(())
    }

    // It basically ignores note on if there's no group. TODO make it have a
    // reference to play the note in case of a proper group is set after the
    // node start
    pub(crate) fn set_note_on(&mut self, pitch: f32) {
        if let Some(group) = &self.group {
            let group = group.clone();
            let mut voice_group = Voice::new(pitch, group);
            voice_group.group.set_note_on(pitch);
            self.voices.push(voice_group);
        }
    }

    pub(crate) fn set_note_off(&mut self, pitch: f32) {
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
