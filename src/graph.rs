use crate::{
    get_items_by_id::get_items_by_id, graph_data::GraphData, modules_by_id::ModulesById,
    set_note_trait::SetNoteTrait, sort::has_id::HasId,
};
use nohash_hasher::IntMap;
use std::error::Error;

// TODO find a better name for this
#[derive(Debug, Default)]
pub struct Graph {
    counter: u32,
    modules_by_id: ModulesById,
    main_module_id: Option<usize>,
}

// TODO consider using a more general approach
const VOICES_REMOTION_CYCLE_SIZE: u32 = 1000;

impl Graph {
    pub fn new() -> Self {
        Graph {
            counter: 0,
            main_module_id: None,
            modules_by_id: IntMap::default(),
        }
    }

    fn update(&mut self, new_graph: GraphData) -> Result<(), Box<dyn Error>> {
        self.main_module_id = new_graph.main_module_id;

        let new_modules = get_items_by_id(new_graph.modules);

        // Remove modules not present in new modules
        self.modules_by_id.retain(|module_id, _| {
            new_modules
                .iter()
                .any(|(new_module_id, _)| new_module_id == module_id)
        });

        for new_module in new_modules.values() {
            // Update a module if present in modules. Saves the new module
            // otherwise
            if let Some(module) = self.modules_by_id.get_mut(&new_module.get_id()) {
                module.update(new_module)?;
            } else {
                let mut module = new_module.clone();
                module.update(new_module)?;
                self.modules_by_id.insert(module.get_id(), module.clone());
            }
        }

        let current_modules = self.modules_by_id.clone();
        for module in self.modules_by_id.values_mut() {
            module.update_modules_in_nodes(&current_modules)?;
        }

        Ok(())
    }

    pub fn update_from_json(&mut self, modules_json: &str) -> Result<(), Box<dyn Error>> {
        let new_graph: GraphData = serde_json::from_str(modules_json)?;
        self.update(new_graph)
    }

    pub fn get_output_value(&self) -> f32 {
        if let Some(main_module_id) = self.main_module_id {
            let main_module = self.modules_by_id.get(&main_module_id).unwrap();
            main_module.get_output_value()
        } else {
            0.
        }
    }

    pub fn process(&mut self) {
        // TODO try to find a most elegant solution than just returning if
        // main_module_id is not present
        if let Some(main_module_id) = self.main_module_id {
            let main_module = self.modules_by_id.get_mut(&main_module_id).unwrap();
            main_module.process();
        }

        self.counter += 1;
        if self.counter > VOICES_REMOTION_CYCLE_SIZE {
            self.counter = 0;
            self.remove_non_pending_voices();
        }
    }

    fn remove_non_pending_voices(&mut self) {
        for module in self.modules_by_id.values_mut() {
            module.remove_non_pending_voices();
        }
    }

    pub fn process_block(&mut self, buffer: &mut [f32], length: usize) {
        for index in 0..length {
            self.process();
            buffer[index] = self.get_output_value();
        }
    }

    pub fn set_note_on(&mut self, pitch: f32) {
        for module in self.modules_by_id.values_mut() {
            module.set_note_on(pitch);
        }
    }

    pub fn set_note_off(&mut self, pitch: f32) {
        for module in self.modules_by_id.values_mut() {
            module.set_note_off(pitch);
        }
    }
}
