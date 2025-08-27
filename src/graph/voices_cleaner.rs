use crate::module::module::Module;

#[derive(Debug, Default, PartialEq)]
pub(crate) struct VoicesCleaner {
    counter: u32,
}

// TODO consider using a more general approach
const VOICES_REMOTION_CYCLE_SIZE: u32 = 1000;

impl VoicesCleaner {
    pub(crate) fn process(&mut self, module: &mut Module) {
        self.counter += 1;
        if self.counter > VOICES_REMOTION_CYCLE_SIZE {
            self.counter = 0;
            module.remove_non_pending_voices();
        }
    }
}
