use super::deserialize_vec_map::deserialize_vec_map;
use crate::{
    control_update_data::ControlUpdateData, declare_get_id, get_inputs_trait::GetInputsTrait,
    has_update::HasUpdate, module::module::Module, node_trait::NodeTrait,
    set_input_indexes_trait::SetInputIndexesTrait, set_note_trait::SetNoteTrait,
    set_sample_rate_trait::SetSampleRateTrait, sort::node_indexes::NodeIndexes, voice::Voice,
};
use serde::Deserialize;
use vector_map::VecMap;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Extras {
    target_module_id: Option<usize>,
    /// Map where the key is the input node id and the value is the target input
    /// from an outer module.
    #[serde(deserialize_with = "deserialize_vec_map")]
    input_target_ids: VecMap<usize, usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct ModuleVoicesNode {
    extras: Extras,
    id: usize,
    #[serde(skip)]
    last_output: f32,
    #[serde(skip)]
    module: Option<Module>,
    #[serde(skip)]
    voices: Vec<Voice>,
}

declare_get_id! {ModuleVoicesNode}

impl ModuleVoicesNode {
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
        for voice in &mut self.voices {
            voice.module.update_control(control_update_data);
        }
    }
}

impl SetInputIndexesTrait for ModuleVoicesNode {
    fn set_input_indexes(&mut self, node_indexes: &NodeIndexes) {
        if let Some(module) = &mut self.module {
            module.set_node_ids_to_indexes(node_indexes, &self.extras.input_target_ids);
        }

        for voice in &mut self.voices {
            voice
                .module
                .set_node_ids_to_indexes(node_indexes, &self.extras.input_target_ids);
        }
    }
}

impl ModuleVoicesNode {
    pub(crate) fn remove_non_pending_voices(&mut self) {
        self.voices.retain(|voice| voice.get_is_pending());
    }
}
impl HasUpdate for ModuleVoicesNode {
    fn update(&mut self, new_module: &Self) {
        self.extras = new_module.extras.clone();
        self.module = new_module.module.clone();

        if let Some(module) = &self.module {
            for voice in &mut self.voices {
                voice.module.update(module);
            }
        } else {
            self.voices = vec![];
        }
    }
}

impl GetInputsTrait for ModuleVoicesNode {
    fn get_input_ids(&self) -> Vec<usize> {
        self.extras.input_target_ids.values().cloned().collect()
    }
}

impl NodeTrait for ModuleVoicesNode {
    fn process(&mut self, node_values: &[f32]) -> f32 {
        let mut sum = 0.;
        for voice in &mut self.voices {
            voice.module.update_input_nodes_values(node_values);
            voice.module.process();
            // TODO check if this makes sense using just the first channel
            let output = voice.get_output_value();
            sum += output;
        }
        self.last_output = sum;
        self.last_output
    }

    fn get_is_pending(&self) -> bool {
        for voice in &self.voices {
            if voice.get_is_pending() {
                return true;
            }
        }
        false
    }
}

impl SetNoteTrait for ModuleVoicesNode {
    // It basically ignores note on if there's no module. TODO make it have a
    // reference to play the note in case of a proper module is set after the
    // node start
    fn set_note_on(&mut self, pitch: f32) {
        if let Some(module) = &self.module {
            let module = module.clone();
            let mut voice = Voice::new(pitch, module);
            voice.set_note_on(pitch);
            self.voices.push(voice);
        }
    }

    fn set_note_off(&mut self, pitch: f32) {
        for voice in &mut self.voices {
            if voice.pitch == pitch {
                voice.set_note_off(pitch);
            }
        }
    }
}

impl SetSampleRateTrait for ModuleVoicesNode {
    fn set_sample_rate(&mut self, sample_rate: f32) {
        if let Some(module) = &mut self.module {
            module.set_sample_rate(sample_rate);
        }
    }
}
