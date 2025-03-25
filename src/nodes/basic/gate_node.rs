use crate::{
    declare_empty_get_input_ids, declare_empty_update, declare_get_id, node_trait::NodeTrait,
    set_note_trait::SetNoteTrait, values_by_id::ValuesById,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct GateNode {
    id: usize,
    #[serde(skip)]
    is_active: bool,
}

declare_get_id! {GateNode}
declare_empty_update! {GateNode}
declare_empty_get_input_ids! {GateNode}

impl NodeTrait for GateNode {
    fn process(&mut self, _node_values: &ValuesById) -> f32 {
        if self.is_active {
            1.
        } else {
            0.
        }
    }
}

impl SetNoteTrait for GateNode {
    fn set_note_on(&mut self, _pitch: f32) {
        self.is_active = true;
    }

    fn set_note_off(&mut self, _pitch: f32) {
        self.is_active = false;
    }
}
