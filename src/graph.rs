use crate::{
    get_items_by_id::get_items_by_id, group::Group, groups_by_id::GroupsById, sort::has_id::HasId,
};
use std::{collections::HashMap, error::Error};

// TODO find a better name for this
#[derive(Debug, Default)]
pub struct Graph {
    groups_by_id: GroupsById,
    main_group_id: Option<usize>,
}

// TODO make polyphonic
impl Graph {
    pub fn new() -> Self {
        Graph {
            main_group_id: None,
            groups_by_id: HashMap::new(),
        }
    }

    pub fn set_groups(&mut self, new_groups: Vec<Group>) -> Result<(), Box<dyn Error>> {
        let new_groups = get_items_by_id(new_groups);

        // Remove groups not present in new groups
        self.groups_by_id.retain(|group_id, _| {
            new_groups
                .iter()
                .any(|(new_group_id, _)| new_group_id == group_id)
        });

        for (_, new_group) in &new_groups {
            // Update a group if present in groups. Saves the new group
            // otherwise
            if let Some(group) = self.groups_by_id.get_mut(&new_group.get_id()) {
                group.update(&new_group)?;
            } else {
                self.groups_by_id
                    .insert(new_group.get_id(), new_group.clone());
            }
        }

        let current_groups = self.groups_by_id.clone();
        for (_, group) in &mut self.groups_by_id {
            group.update_groups_in_nodes(&current_groups)?;
        }

        Ok(())
    }

    pub fn set_groups_from_json(&mut self, groups_json: &str) -> Result<(), Box<dyn Error>> {
        let new_groups: Vec<Group> = serde_json::from_str(groups_json)?;
        self.set_groups(new_groups)
    }

    pub fn get_output_value(&self) -> f32 {
        if let Some(main_group_id) = self.main_group_id {
            let main_group = self.groups_by_id.get(&main_group_id).unwrap();
            main_group.get_output_value()
        } else {
            return 0.;
        }
    }

    pub fn process(&mut self) {
        // TODO try to find a most elegant solution than just returning if
        // main_group_id is not present
        if let Some(main_group_id) = self.main_group_id {
            let main_group = self.groups_by_id.get_mut(&main_group_id).unwrap();
            main_group.process();
        }
    }

    pub fn process_block(&mut self, buffer: &mut [f32], length: usize) {
        for index in 0..length {
            self.process();
            buffer[index] = self.get_output_value();
        }
    }

    pub fn set_note_on(&mut self, pitch: f32) {
        for (_, group) in &mut self.groups_by_id {
            group.set_note_on(pitch);
        }
    }

    pub fn set_note_off(&mut self, pitch: f32) {
        for (_, group) in &mut self.groups_by_id {
            group.set_note_off(pitch);
        }
    }

    pub fn set_main_group_id(&mut self, main_group_id: usize) {
        self.main_group_id = Some(main_group_id)
    }
}
