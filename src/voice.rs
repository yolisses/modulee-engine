use crate::{
    control_update_data::ControlUpdateData, module::module::Module, set_note_trait::SetNoteTrait,
};
use nohash_hasher::IntMap;

// TODO check if all these derives make sense to be used here
#[derive(Debug, Clone)]
pub(crate) struct Voice {
    module: Module,
    pub(crate) pitch: f32,
    pub(crate) is_waiting_note_off: bool,
}

impl Voice {
    pub(crate) fn new(pitch: f32, module: Module) -> Self {
        Self {
            pitch,
            module,
            is_waiting_note_off: false,
        }
    }

    pub(crate) fn get_is_pending(&self) -> bool {
        if self.is_waiting_note_off {
            return true;
        };
        self.module.get_is_pending()
    }

    pub(crate) fn get_output_value(&self) -> f32 {
        if self.get_is_pending() {
            return self.module.get_output_value();
        }
        0.
    }

    // TODO consider making module public
    pub(crate) fn process(&mut self) {
        self.module.process();
    }

    pub(crate) fn update_module(&mut self, new_module: &Module) {
        self.module.update(new_module);
    }

    pub(crate) fn set_input_node_values(
        &mut self,
        node_values: &[f32],
        input_target_ids: &IntMap<usize, usize>,
    ) {
        self.module
            .set_input_node_values(node_values, input_target_ids);
    }

    pub(crate) fn update_control(&mut self, control_update_data: &ControlUpdateData) {
        self.module.update_control(control_update_data)
    }
}

impl SetNoteTrait for Voice {
    fn set_note_on(&mut self, pitch: f32) {
        self.is_waiting_note_off = true;
        self.module.set_note_on(pitch);
    }

    fn set_note_off(&mut self, pitch: f32) {
        self.is_waiting_note_off = false;
        self.module.set_note_off(pitch);
    }
}
