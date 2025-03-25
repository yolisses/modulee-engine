use super::deserialize_int_map::deserialize_int_map;
use crate::{
    get_updated_module::get_updated_module, module::Module, node_trait::NodeTrait,
    set_note_trait::SetNoteTrait, values_by_id::ValuesById,
};
use nohash_hasher::IntMap;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Extras {
    target_module_id: Option<usize>,
    #[serde(deserialize_with = "deserialize_int_map")]
    input_target_ids: IntMap<usize, usize>,
}

/// Returns the phase value between 0 and 1 given a time and a frequency
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct ModuleNode {
    id: usize,
    extras: Extras,
    #[serde(skip)]
    module: Option<Module>,
}

impl ModuleNode {
    pub(crate) fn update_modules(
        &mut self,
        new_modules: &IntMap<usize, Module>,
    ) -> Result<(), Box<dyn Error>> {
        self.module = get_updated_module(
            self.module.take(),
            self.extras.target_module_id,
            new_modules,
        )?;
        Ok(())
    }

    pub(crate) fn set_note_on(&mut self, pitch: f32) {
        if let Some(module) = &mut self.module {
            module.set_note_on(pitch);
        }
    }

    pub(crate) fn set_note_off(&mut self, pitch: f32) {
        if let Some(module) = &mut self.module {
            module.set_note_off(pitch);
        }
    }
}

impl NodeTrait for ModuleNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        if let Some(module) = &mut self.module {
            module.update_input_nodes(node_values, &self.extras.input_target_ids);
            module.process();
            // TODO use all outputs
            module.get_output_value()
        } else {
            0.
        }
    }

    fn get_input_ids(&self) -> Vec<usize> {
        self.extras.input_target_ids.values().cloned().collect()
    }

    fn update(&mut self, new_node: &Self) {
        self.extras = new_node.extras.clone();
    }
}
