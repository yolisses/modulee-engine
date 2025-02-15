use crate::{
    envelope::envelope::Envelope, node_trait::NodeTrait, set_note_trait::SetNoteTrait,
    sort::has_id::HasId, values_by_id::ValuesById,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct EnvelopeNode {
    id: usize,
    #[serde(skip)]
    #[serde(default = "get_default_envelope")]
    envelope: Envelope,
}

fn get_default_envelope() -> Envelope {
    Envelope::new(0., 0., 0., 0., 0.)
}

impl NodeTrait for EnvelopeNode {
    fn process(&mut self, _node_values: &ValuesById) -> f32 {
        // TODO update envelope params
        self.envelope.process();
        self.envelope.get_value()
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![]
    }
}

impl HasId for EnvelopeNode {
    fn get_id(&self) -> usize {
        self.id
    }
}

impl SetNoteTrait for EnvelopeNode {
    fn set_note_on(&mut self, _pitch: f32) {
        self.envelope.set_note_on();
    }

    fn set_note_off(&mut self, _pitch: f32) {
        self.envelope.set_note_off();
    }
}
