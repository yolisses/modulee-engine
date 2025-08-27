use crate::{
    graph::voices_cleaner::VoicesCleaner, module::module::Module, set_note_trait::SetNoteTrait,
    set_sample_rate_trait::SetSampleRateTrait,
};

#[derive(Debug, Default, PartialEq)]
pub struct Graph {
    voices_cleaner: VoicesCleaner,
    pub(crate) main_module: Option<Module>,
}

impl Graph {
    pub fn process(&mut self) {
        if let Some(main_module) = &mut self.main_module {
            main_module.process();
            self.voices_cleaner.process(main_module);
        }
    }

    pub fn get_output_values(&self) -> (f32, f32) {
        if let Some(main_module) = &self.main_module {
            main_module.get_output_values()
        } else {
            (0., 0.)
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
        if let Some(main_module) = &mut self.main_module {
            main_module.set_sample_rate(sample_rate);
        }
    }
}
