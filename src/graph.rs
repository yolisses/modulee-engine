use crate::{
    get_items_by_id::get_items_by_id, graph_data::GraphData, groups_by_id::GroupsById,
    set_note_trait::SetNoteTrait, sort::has_id::HasId,
};
use nohash_hasher::IntMap;
use std::error::Error;

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
            groups_by_id: IntMap::default(),
        }
    }

    fn update(&mut self, new_graph: GraphData) -> Result<(), Box<dyn Error>> {
        self.main_group_id = new_graph.main_group_id;

        let new_groups = get_items_by_id(new_graph.groups);

        // Remove groups not present in new groups
        self.groups_by_id.retain(|group_id, _| {
            new_groups
                .iter()
                .any(|(new_group_id, _)| new_group_id == group_id)
        });

        for new_group in new_groups.values() {
            // Update a group if present in groups. Saves the new group
            // otherwise
            if let Some(group) = self.groups_by_id.get_mut(&new_group.get_id()) {
                group.update(new_group)?;
            } else {
                let mut group = new_group.clone();
                group.update(new_group)?;
                self.groups_by_id.insert(group.get_id(), group.clone());
            }
        }

        let current_groups = self.groups_by_id.clone();
        for group in self.groups_by_id.values_mut() {
            group.update_groups_in_nodes(&current_groups)?;
        }

        Ok(())
    }

    pub fn update_from_json(&mut self, groups_json: &str) -> Result<(), Box<dyn Error>> {
        let new_graph: GraphData = serde_json::from_str(groups_json)?;
        self.update(new_graph)
    }

    pub fn get_output_value(&self) -> f32 {
        if let Some(main_group_id) = self.main_group_id {
            let main_group = self.groups_by_id.get(&main_group_id).unwrap();
            main_group.get_output_value()
        } else {
            0.
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
        for group in self.groups_by_id.values_mut() {
            group.set_note_on(pitch);
        }
    }

    pub fn set_note_off(&mut self, pitch: f32) {
        for group in self.groups_by_id.values_mut() {
            group.set_note_off(pitch);
        }
    }
}
