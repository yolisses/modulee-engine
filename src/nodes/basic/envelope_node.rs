use crate::{
    envelope::envelope::Envelope, node_trait::NodeTrait, sample_rate::SAMPLE_RATE,
    set_note_trait::SetNoteTrait, sort::has_id::HasId, values_by_id::ValuesById,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct InputIds {
    attack: usize,
    decay: usize,
    sustain: usize,
    release: usize,
}

// TODO consider adding gate, to allow envelope starts different than note
// starts.
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct EnvelopeNode {
    id: usize,
    input_ids: InputIds,
    #[serde(skip)]
    #[serde(default = "get_default_envelope")]
    envelope: Envelope,
}

fn get_default_envelope() -> Envelope {
    Envelope::new(1., 1., 1., 1., SAMPLE_RATE)
}

impl NodeTrait for EnvelopeNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        let attack = node_values[&self.input_ids.attack];
        let decay = node_values[&self.input_ids.decay];
        let sustain = node_values[&self.input_ids.sustain];
        let release = node_values[&self.input_ids.release];
        self.envelope
            .update_parameters(attack, decay, sustain, release);

        self.envelope.process();
        self.envelope.get_value()
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![
            self.input_ids.attack,
            self.input_ids.decay,
            self.input_ids.sustain,
            self.input_ids.release,
        ]
    }

    // TODO check if makes sense to clone the envelope too
    fn update(&mut self, new_node: &Self) {
        self.input_ids = new_node.input_ids.clone();
    }

    fn get_is_pending(&self) -> bool {
        self.envelope.get_is_pending()
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
