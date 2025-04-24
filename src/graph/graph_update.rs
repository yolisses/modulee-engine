use super::{graph::Graph, graph_data::GraphData};
use crate::{get_items_by_id::get_items_by_id, sort::has_id::HasId};
use std::error::Error;

impl Graph {
    fn update(&mut self, new_graph_data: GraphData) -> Result<(), Box<dyn Error>> {
        self.main_module_id = new_graph_data.main_module_id;

        let new_modules = get_items_by_id(new_graph_data.modules);

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
        let mut new_graph_data: GraphData = serde_json::from_str(modules_json)?;
        new_graph_data.prepare();
        self.update(new_graph_data)
    }
}
