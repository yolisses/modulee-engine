use crate::group::Group;
use serde::Deserialize;

// TODO check if all these derives make sense to be used here
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Voice {
    pub(crate) pitch: f32,
    pub(crate) group: Group,
}

impl Voice {
    pub(crate) fn new(pitch: f32, group: Group) -> Self {
        Self { pitch, group }
    }
}
