use super::{graph::Graph, graph_data::GraphData};
use crate::{control_update_data::ControlUpdateData, sort::has_id::HasId};
use std::error::Error;

impl Graph {
    fn update(&mut self, new_graph_data: GraphData) {
        let main_module_id = new_graph_data.main_module_id;

        if let Some(main_module_id) = main_module_id {
            let new_main_module = new_graph_data
                .modules
                .iter()
                .find(|module| module.get_id() == main_module_id);

            if let Some(new_main_module) = new_main_module {
                if let Some(main_module) = &mut self.main_module {
                    if main_module.get_id() == main_module_id {
                        main_module.update(&new_main_module);
                    } else {
                        self.main_module = Some(new_main_module.clone());
                    }
                } else {
                    self.main_module = Some(new_main_module.clone());
                }
            } else {
                self.main_module = None;
            }
        } else {
            self.main_module = None;
        }
    }

    pub fn update_from_json(&mut self, modules_json: &str) -> Result<(), Box<dyn Error>> {
        let mut new_graph_data: GraphData = serde_json::from_str(modules_json)?;
        new_graph_data.prepare()?;
        self.update(new_graph_data);
        Ok(())
    }

    pub fn update_control(&mut self, control_update_data: &ControlUpdateData) {
        if let Some(main_module) = &mut self.main_module {
            main_module.update_control(control_update_data);
        }
    }
}
