use crate::{group::Group, set_note_trait::SetNoteTrait};

// TODO check if all these derives make sense to be used here
#[derive(Debug, Clone)]
pub(crate) struct Voice {
    pub(crate) pitch: f32,
    pub(crate) group: Group,
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
}

impl SetNoteTrait for Voice {
    fn set_note_on(&mut self, pitch: f32) {
        self.is_waiting_note_off = true;
        self.group.set_note_on(pitch);
    }

    fn set_note_off(&mut self, pitch: f32) {
        if self.pitch == pitch {
            self.is_waiting_note_off = false;
            self.group.set_note_off(pitch);
        }
    }
}
