use crate::{
    module::module::Module, set_note_trait::SetNoteTrait, set_sample_rate_trait::SetSampleRateTrait,
};

// TODO consider storing only the main module, since the updates are data
// complete.
#[derive(Debug, Default, PartialEq)]
pub struct Graph {
    counter: u32,
    pub(crate) main_module: Option<Module>,
}

// TODO consider using a more general approach
const VOICES_REMOTION_CYCLE_SIZE: u32 = 1000;

impl Graph {
    fn remove_non_pending_voices(&mut self) {
        if let Some(main_module) = &mut self.main_module {
            main_module.remove_non_pending_voices();
        }
    }

    pub fn process(&mut self) {
        // TODO try to find a most elegant solution than just returning if
        // main_module_id is not present
        if let Some(main_module) = &mut self.main_module {
            main_module.process();

            self.counter += 1;
            if self.counter > VOICES_REMOTION_CYCLE_SIZE {
                self.counter = 0;
                self.remove_non_pending_voices();
            }
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
