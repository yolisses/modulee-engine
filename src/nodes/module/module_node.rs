use super::deserialize_int_map::deserialize_int_map;
use crate::{
    control_update_data::ControlUpdateData, declare_get_id, get_inputs_trait::GetInputsTrait,
    has_update::HasUpdate, module::module::Module, node_trait::NodeTrait,
    set_input_indexes_trait::SetInputIndexesTrait, set_note_trait::SetNoteTrait,
    sort::node_indexes::NodeIndexes,
};
use nohash_hasher::IntMap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Extras {
    target_module_id: Option<usize>,
    #[serde(deserialize_with = "deserialize_int_map")]
    input_target_ids: IntMap<usize, usize>,
}

/// Returns the phase value between 0 and 1 given a time and a frequency
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct ModuleNode {
    extras: Extras,
    id: usize,
    #[serde(skip)]
    last_outputs: (f32, f32),
    #[serde(skip)]
    module: Option<Module>,
}

impl ModuleNode {
    pub(crate) fn get_last_outputs(&self) -> (f32, f32) {
        self.last_outputs
    }

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

declare_get_id! {ModuleNode}

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

impl SetInputIndexesTrait for ModuleNode {
    fn set_input_indexes(&mut self, node_indexes: &NodeIndexes) {
        let updates: Vec<(usize, usize)> = self
            .extras
            .input_target_ids
            .iter()
            .map(|(input_id, target_id)| (*input_id, node_indexes[target_id]))
            .collect();

        for (input_id, index) in updates {
            self.extras.input_target_ids.insert(input_id, index);
        }
    }
}

impl NodeTrait for ModuleNode {
    fn process(&mut self, node_values: &[f32]) -> f32 {
        if let Some(module) = &mut self.module {
            module.set_input_node_values(node_values, &self.extras.input_target_ids);
            module.process();
            self.last_outputs = module.get_output_values();
            self.last_outputs.0
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
