use crate::{get_items_by_id::get_items_by_id, group::Group, groups_by_id::GroupsById};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Graph {
    main_group_id: usize,
    groups_by_id: GroupsById,
}

// TODO make polyphonic
impl Graph {
    pub fn new(main_group_id: usize) -> Self {
        Graph {
            main_group_id,
            groups_by_id: HashMap::new(),
        }
    }

    pub fn set_groups_from_json(
        &mut self,
        groups_json: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut groups: Vec<Group> = serde_json::from_str(groups_json)?;

        for group in &mut groups {
            group.sort_nodes_topologically()?
        }

        self.groups_by_id = get_items_by_id(groups);
        println!("Groups: {:?}", self.groups_by_id);
        Ok(())
    }

    pub fn get_output_value(&self) -> f32 {
        let main_group = self.groups_by_id.get(&self.main_group_id).unwrap();
        main_group.get_output_value()
    }

    pub fn process(&mut self) {
        let main_group = self.groups_by_id.get_mut(&self.main_group_id).unwrap();
        main_group.process();
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
}
