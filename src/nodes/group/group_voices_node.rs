use crate::{
    get_updated_group::get_updated_group, group::Group, node_trait::NodeTrait,
    set_note_trait::SetNoteTrait, sort::has_id::HasId, values_by_id::ValuesById, voice::Voice,
};
use nohash_hasher::IntMap;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Extras {
    target_group_id: Option<usize>,
    input_target_ids: IntMap<usize, usize>,
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
        new_groups: &IntMap<usize, Group>,
    ) -> Result<(), Box<dyn Error>> {
        self.group = get_updated_group(self.group.take(), self.extras.target_group_id, new_groups)?;
        Ok(())
    }

    pub(crate) fn remove_non_pending_voices(&mut self) {
        self.voices.retain(|voice| voice.get_is_pending());
    }
}

impl NodeTrait for GroupVoicesNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        let mut sum = 0.;
        for voice in &mut self.voices {
            voice.update_input_nodes(node_values, &self.extras.input_target_ids);
            voice.process();
            sum += voice.get_output_value()
        }
        sum
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![]
    }

    fn update(&mut self, new_node: &Self) {
        self.extras = new_node.extras.clone();
    }
}

impl HasId for GroupVoicesNode {
    fn get_id(&self) -> usize {
        self.id
    }
}

impl SetNoteTrait for GroupVoicesNode {
    // It basically ignores note on if there's no group. TODO make it have a
    // reference to play the note in case of a proper group is set after the
    // node start
    fn set_note_on(&mut self, pitch: f32) {
        if let Some(group) = &self.group {
            let group = group.clone();
            let mut voice = Voice::new(pitch, group);
            voice.set_note_on(pitch);
            self.voices.push(voice);
        }
    }

    fn set_note_off(&mut self, pitch: f32) {
        for voice in &mut self.voices {
            voice.set_note_off(pitch);
        }
    }
}
