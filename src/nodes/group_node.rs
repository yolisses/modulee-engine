use crate::{group::Group, node_trait::NodeTrait, sort::has_id::HasId, values_by_id::ValuesById};
use serde::Deserialize;
use std::{collections::HashMap, error::Error};

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Extras {
    target_group_id: usize,
    input_target_ids: HashMap<usize, usize>,
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
        new_groups: &HashMap<usize, Group>,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(new_group) = new_groups.get(&self.extras.target_group_id) {
            if let Some(group) = &mut self.group {
                group.update(new_group)?;
            } else {
                self.group = Some(new_group.clone());
            }
        } else {
            panic!(
                "Can't find a group with id {} to update group node with id {}",
                self.extras.target_group_id,
                self.get_id()
            )
        }
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
