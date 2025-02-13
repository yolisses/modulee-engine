use crate::{
    node::Node,
    node_trait::NodeTrait,
    sort::{has_id::HasId, sort_nodes_topologically::sort_nodes_topologically},
    values_by_id::ValuesById,
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Group {
    id: usize,
    nodes: Vec<Node>,
    #[serde(skip)]
    last_pitch: f32,
}

impl PartialEq for Group {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl HasId for Group {
    fn get_id(&self) -> usize {
        self.id
    }
}

// TODO make polyphonic
impl Group {
    pub(crate) fn sort_nodes_topologically(&mut self) -> Result<(), String> {
        sort_nodes_topologically(&mut self.nodes)
    }

    // This may be a violation of the responsibility division, but improves
    // performance
    pub(crate) fn update_input_nodes(
        &mut self,
        node_values: &ValuesById,
        input_target_ids: &HashMap<usize, usize>,
    ) {
        for node in &mut self.nodes {
            if let Node::InputNode(input_node) = node {
                let target_id = input_target_ids[&input_node.get_id()];
                let value = node_values[&target_id];
                input_node.set_value(value);
            }
        }
    }

    pub fn get_output_value(&self) -> f32 {
        for node in &self.nodes {
            match node {
                Node::OutputNode(output_node) => return output_node.get_value(),
                _ => (),
            }
        }
        0.
    }

    pub fn process(&mut self) {
        let mut node_values = ValuesById::new();
        for node in &mut self.nodes {
            let value = node.process(&node_values);
            node_values.insert(node.get_id(), value);
        }
    }

    pub(crate) fn update(&mut self, other_group: &Self) {}

    pub fn set_note_on(&mut self, pitch: f32) {
        for node in &mut self.nodes {
            match node {
                Node::PitchNode(pitch_node) => {
                    pitch_node.set_pitch(pitch);
                }
                Node::GateNode(gate_node) => {
                    gate_node.set_is_active(true);
                }
                Node::GroupVoicesNode(voice_group_node) => {
                    voice_group_node.set_note_on(pitch);
                }
                _ => (),
            }
        }
        self.last_pitch = pitch;
    }

    pub fn set_note_off(&mut self, pitch: f32) {
        if self.last_pitch != pitch {
            return;
        }
        for node in &mut self.nodes {
            match node {
                Node::GateNode(gate_node) => {
                    gate_node.set_is_active(false);
                }
                Node::GroupVoicesNode(voice_group_node) => {
                    voice_group_node.set_note_off(pitch);
                }
                _ => (),
            }
        }
    }
}
