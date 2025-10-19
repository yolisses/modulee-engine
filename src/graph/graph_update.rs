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
                if let Some(main_module_instances) = &mut self.main_module_instances {
                    if main_module_instances[0].get_id() == main_module_id {
                        for main_module_instance in main_module_instances {
                            main_module_instance.update(new_main_module);
                        }
                    } else {
                        self.main_module_instances =
                            Some([new_main_module.clone(), new_main_module.clone()]);
                    }
                } else {
                    self.main_module_instances =
                        Some([new_main_module.clone(), new_main_module.clone()]);
                }
            } else {
                self.main_module_instances = None;
            }
        } else {
            self.main_module_instances = None;
        }
    }

    pub fn update_from_json(&mut self, modules_json: &str) -> Result<(), Box<dyn Error>> {
        let mut new_graph_data: GraphData = serde_json::from_str(modules_json)?;
        new_graph_data.prepare(self.sample_rate)?;
        self.update(new_graph_data);
        Ok(())
    }

    pub fn update_control(&mut self, control_update_data: &ControlUpdateData) {
        if let Some(main_module_instances) = &mut self.main_module_instances {
            main_module_instances[0].update_control(control_update_data);
            main_module_instances[1].update_control(control_update_data);
        }
    }
}
