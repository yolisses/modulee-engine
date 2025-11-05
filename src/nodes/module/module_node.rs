use super::deserialize_vec_map::deserialize_vec_map;
use crate::{
    control_update_data::ControlUpdateData, declare_get_id, get_inputs_trait::GetInputsTrait,
    has_update::HasUpdate, module::module::Module, node_trait::NodeTrait,
    set_input_indexes_trait::SetInputIndexesTrait, set_note_trait::SetNoteTrait,
    set_sample_rate_trait::SetSampleRateTrait, sort::node_indexes::NodeIndexes,
};
use serde::Deserialize;
use vector_map::VecMap;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Extras {
    target_module_id: Option<usize>,
    /// Map where the key is the input node id and the value is the target node
    /// from an outer module.
    #[serde(deserialize_with = "deserialize_vec_map")]
    input_target_ids: VecMap<usize, usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct ModuleNode {
    extras: Extras,
    id: usize,
    #[serde(skip)]
    last_output: f32,
    #[serde(skip)]
    module: Option<Module>,
}

declare_get_id! {ModuleNode}

impl ModuleNode {
    pub(crate) fn prepare_module(&mut self, possible_modules: &[Module]) {
        if let Some(target_module_id) = self.extras.target_module_id {
            let module = possible_modules
                .iter()
                .find(|module| module.get_id() == target_module_id);
            if let Some(module) = module {
                self.module = Some(module.clone())
            } else {
                self.module = None
            }
        } else {
            self.module = None
        }
    }

    pub(crate) fn get_target_module_id(&self) -> Option<usize> {
        self.extras.target_module_id
    }

    pub(crate) fn update_control(&mut self, control_update_data: &ControlUpdateData) {
        if let Some(module) = &mut self.module {
            module.update_control(control_update_data)
        }
    }
}

impl SetInputIndexesTrait for ModuleNode {
    fn set_input_indexes(&mut self, _node_indexes: &NodeIndexes) {
        if let Some(module) = &mut self.module {
            module.set_node_ids_to_indexes();
        }
    }
}

impl HasUpdate for ModuleNode {
    fn update(&mut self, new_module: &Self) {
        self.extras = new_module.extras.clone();
        self.module = new_module.module.clone();
    }
}

impl GetInputsTrait for ModuleNode {
    fn get_input_ids(&self) -> Vec<usize> {
        self.extras.input_target_ids.values().cloned().collect()
    }
}

impl NodeTrait for ModuleNode {
    fn process(&mut self, node_values: &[f32]) -> f32 {
        if let Some(module) = &mut self.module {
            module.update_input_nodes_values(node_values);
            module.process();
            self.last_output = module.get_output_value();
            self.last_output
        } else {
            0.
        }
    }
}

impl SetNoteTrait for ModuleNode {
    fn set_note_on(&mut self, pitch: f32) {
        if let Some(module) = &mut self.module {
            module.set_note_on(pitch);
        }
    }

    fn set_note_off(&mut self, pitch: f32) {
        if let Some(module) = &mut self.module {
            module.set_note_off(pitch);
        }
    }
}

impl SetSampleRateTrait for ModuleNode {
    fn set_sample_rate(&mut self, sample_rate: f32) {
        if let Some(module) = &mut self.module {
            module.set_sample_rate(sample_rate);
        }
    }
}
