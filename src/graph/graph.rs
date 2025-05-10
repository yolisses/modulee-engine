use crate::{module::modules_by_id::ModulesById, set_note_trait::SetNoteTrait};

// TODO consider storing only the main module, since the updates are data
// complete.
#[derive(Debug, Default, PartialEq)]
pub struct Graph {
    counter: u32,
    pub(crate) modules_by_id: ModulesById,
    pub(crate) main_module_id: Option<usize>,
}

// TODO consider using a more general approach
const VOICES_REMOTION_CYCLE_SIZE: u32 = 1000;

impl Graph {
    pub fn get_output_value(&self) -> f32 {
        if let Some(main_module_id) = self.main_module_id {
            let main_module = self.modules_by_id.get(&main_module_id).unwrap();
            main_module.get_output_value()
        } else {
            0.
        }
    }

    fn remove_non_pending_voices(&mut self) {
        for module in self.modules_by_id.values_mut() {
            module.remove_non_pending_voices();
        }
    }

    pub fn process(&mut self) {
        // TODO try to find a most elegant solution than just returning if
        // main_module_id is not present
        if let Some(main_module_id) = self.main_module_id {
            let main_module = self.modules_by_id.get_mut(&main_module_id).unwrap();
            main_module.process();
        }

        self.counter += 1;
        if self.counter > VOICES_REMOTION_CYCLE_SIZE {
            self.counter = 0;
            self.remove_non_pending_voices();
        }
    }

    pub fn process_block(&mut self, buffer: &mut [f32], length: usize) {
        for value in buffer.iter_mut().take(length) {
            self.process();
            *value = self.get_output_value();
        }
    }

    pub fn set_note_on(&mut self, pitch: f32) {
        for module in self.modules_by_id.values_mut() {
            module.set_note_on(pitch);
        }
    }

    pub fn set_note_off(&mut self, pitch: f32) {
        for module in self.modules_by_id.values_mut() {
            module.set_note_off(pitch);
        }
    }
}
