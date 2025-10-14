use crate::{
    graph::voices_cleaner::VoicesCleaner, module::module::Module, set_note_trait::SetNoteTrait,
};

#[derive(Debug, PartialEq)]
pub struct Graph {
    pub(crate) main_module: Option<Module>,
    pub(crate) sample_rate: f32,
    voices_cleaner: VoicesCleaner,
    empty_vector: Vec<f32>,
}

impl Graph {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            empty_vector: Default::default(),
            main_module: Default::default(),
            sample_rate,
            voices_cleaner: Default::default(),
        }
    }

    pub fn process(&mut self) {
        if let Some(main_module) = &mut self.main_module {
            main_module.process(&self.empty_vector);
            self.voices_cleaner.process(main_module);
        }
    }

    pub fn get_output_values(&self) -> f32 {
        if let Some(main_module) = &self.main_module {
            main_module.get_output_value()
        } else {
            0.
        }
    }

    pub fn set_note_on(&mut self, pitch: f32) {
        if let Some(main_module) = &mut self.main_module {
            main_module.set_note_on(pitch);
        }
    }

    pub fn set_note_off(&mut self, pitch: f32) {
        if let Some(main_module) = &mut self.main_module {
            main_module.set_note_off(pitch);
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        if let Some(main_module) = &mut self.main_module {
            main_module.set_sample_rate(sample_rate);
        }
    }
}
