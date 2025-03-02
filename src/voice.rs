use crate::{group::Group, set_note_trait::SetNoteTrait, values_by_id::ValuesById};
use nohash_hasher::IntMap;

// TODO check if all these derives make sense to be used here
#[derive(Debug, Clone)]
pub(crate) struct Voice {
    group: Group,
    pub(crate) pitch: f32,
    pub(crate) is_waiting_note_off: bool,
}

impl Voice {
    pub(crate) fn new(pitch: f32, group: Group) -> Self {
        Self {
            pitch,
            group,
            is_waiting_note_off: false,
        }
    }

    pub(crate) fn get_is_pending(&self) -> bool {
        if self.is_waiting_note_off {
            return true;
        };
        self.group.get_is_pending()
    }

    pub(crate) fn update_input_nodes(
        &mut self,
        node_values: &ValuesById,
        input_target_ids: &IntMap<usize, usize>,
    ) {
        self.group.update_input_nodes(node_values, input_target_ids);
    }

    pub(crate) fn get_output_value(&self) -> f32 {
        if self.get_is_pending() {
            return self.group.get_output_value();
        }
        0.
    }

    pub(crate) fn process(&mut self) {
        self.group.process();
    }
}

impl SetNoteTrait for Voice {
    fn set_note_on(&mut self, pitch: f32) {
        self.is_waiting_note_off = true;
        self.group.set_note_on(pitch);
    }

    fn set_note_off(&mut self, pitch: f32) {
        self.is_waiting_note_off = false;
        self.group.set_note_off(pitch);
    }
}
