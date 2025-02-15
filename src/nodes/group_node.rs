use crate::{
    get_updated_group::get_updated_group, group::Group, node_trait::NodeTrait,
    set_note_trait::SetNoteTrait, sort::has_id::HasId, values_by_id::ValuesById,
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
pub(crate) struct GroupNode {
    id: usize,
    extras: Extras,
    #[serde(skip)]
    group: Option<Group>,
}

impl GroupNode {
    pub(crate) fn update_groups(
        &mut self,
        new_groups: &IntMap<usize, Group>,
    ) -> Result<(), Box<dyn Error>> {
        self.group = get_updated_group(self.group.take(), self.extras.target_group_id, new_groups)?;
        Ok(())
    }

    pub(crate) fn set_note_on(&mut self, pitch: f32) {
        if let Some(group) = &mut self.group {
            group.set_note_on(pitch);
        }
    }

    pub(crate) fn set_note_off(&mut self, pitch: f32) {
        if let Some(group) = &mut self.group {
            group.set_note_off(pitch);
        }
    }
}

impl NodeTrait for GroupNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        if let Some(group) = &mut self.group {
            group.update_input_nodes(node_values, &self.extras.input_target_ids);
            group.process();
            // TODO use all outputs
            group.get_output_value()
        } else {
            0.
        }
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![]
    }
}

impl HasId for GroupNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
