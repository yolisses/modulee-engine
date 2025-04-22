use crate::{
    declare_empty_get_input_ids, declare_empty_update, declare_get_id, node_trait::NodeTrait,
    set_note_trait::SetNoteTrait, values_by_id::ValuesById,
};
use nohash_hasher::IntSet;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct GateNode {
    id: usize,
    #[serde(skip)]
    is_active: bool,
    #[serde(skip)]
    active_pitches: IntSet<i32>,
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
    fn set_note_on(&mut self, pitch: f32) {
        self.active_pitches.insert(pitch as i32);
        self.is_active = true;
    }

    fn set_note_off(&mut self, pitch: f32) {
        self.active_pitches.remove(&(pitch as i32));
        self.is_active = !self.active_pitches.is_empty();
    }
}

#[cfg(test)]
mod tests {
    use crate::{nodes::basic::gate_node::GateNode, set_note_trait::SetNoteTrait};

    #[test]
    fn test_set_notes_on_and_off() {
        let mut gate_node = GateNode {
            id: 1,
            is_active: false,
            active_pitches: Default::default(),
        };

        assert_eq!(gate_node.is_active, false);

        gate_node.set_note_on(1.);
        assert_eq!(gate_node.is_active, true);

        gate_node.set_note_off(1.);
        assert_eq!(gate_node.is_active, false);

        gate_node.set_note_on(2.);
        assert_eq!(gate_node.is_active, true);

        gate_node.set_note_on(3.);
        assert_eq!(gate_node.is_active, true);

        gate_node.set_note_off(2.);
        assert_eq!(gate_node.is_active, true);

        gate_node.set_note_off(3.);
        assert_eq!(gate_node.is_active, false);
    }
}
