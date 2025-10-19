use crate::{
    graph::voices_cleaner::VoicesCleaner, module::module::Module, set_note_trait::SetNoteTrait,
};

#[derive(Debug, PartialEq)]
pub struct Graph {
    empty_vector: Vec<f32>,
    pub(crate) main_module_instances: Option<[Module; 2]>,
    pub(crate) sample_rate: f32,
    voices_cleaner: VoicesCleaner,
}

impl Graph {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            empty_vector: Default::default(),
            main_module_instances: Default::default(),
            sample_rate,
            voices_cleaner: Default::default(),
        }
    }

    pub fn process(&mut self) {
        if let Some(main_module_instances) = &mut self.main_module_instances {
            main_module_instances[0].process(&self.empty_vector);
            main_module_instances[1].process(&self.empty_vector);
            self.voices_cleaner.process(&mut main_module_instances[0]);
            self.voices_cleaner.process(&mut main_module_instances[1]);
        }
    }

    pub fn get_output_values(&self) -> (f32, f32) {
        if let Some(main_module) = &self.main_module_instances {
            (
                main_module[0].get_output_value(),
                main_module[1].get_output_value(),
            )
        } else {
            (0., 0.)
        }
    }

    pub fn set_note_on(&mut self, pitch: f32) {
        if let Some(main_module_instances) = &mut self.main_module_instances {
            main_module_instances[0].set_note_on(pitch);
            main_module_instances[1].set_note_on(pitch);
        }
    }

    pub fn set_note_off(&mut self, pitch: f32) {
        if let Some(main_module_instances) = &mut self.main_module_instances {
            main_module_instances[0].set_note_off(pitch);
            main_module_instances[1].set_note_off(pitch);
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;

        if let Some(main_module_instances) = &mut self.main_module_instances {
            main_module_instances[0].set_sample_rate(sample_rate);
            main_module_instances[1].set_sample_rate(sample_rate);
        }
    }
}
