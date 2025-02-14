use crate::{
    node::Node,
    node_trait::NodeTrait,
    sort::{has_id::HasId, sort_nodes_topologically::sort_nodes_topologically},
    values_by_id::ValuesById,
};
use serde::Deserialize;
use std::{collections::HashMap, error::Error};

#[derive(Debug, Deserialize, Clone)]
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

    pub(crate) fn get_output_value(&self) -> f32 {
        for node in &self.nodes {
            if let Node::OutputNode(output_node) = node { return output_node.get_value() }
        }
        0.
    }

    pub(crate) fn process(&mut self) {
        let mut node_values = ValuesById::new();
        for node in &mut self.nodes {
            let value = node.process(&node_values);
            node_values.insert(node.get_id(), value);
        }
    }

    pub(crate) fn sort_nodes_topologically(&mut self) -> Result<(), Box<dyn Error>> {
        sort_nodes_topologically(&mut self.nodes)?;
        Ok(())
    }

    pub(crate) fn update(&mut self, new_group: &Group) -> Result<(), Box<dyn Error>> {
        let new_nodes = &new_group.nodes;

        // Remove nodes not present in new groups
        self.nodes.retain(|node| {
            new_nodes
                .iter()
                .any(|new_node| new_node.get_id() == node.get_id())
        });

        for new_node in new_nodes {
            let id = new_node.get_id();
            // Update a node if present in nodes. Saves a copy of the new node
            // otherwise
            let node_option = self.nodes.iter_mut().find(|node| node.get_id() == id);
            if let Some(node) = node_option {
                node.update(new_node)?;
            } else {
                self.nodes.push(new_node.clone());
            }
        }

        self.sort_nodes_topologically()?;
        Ok(())
    }

    pub(crate) fn update_groups_in_nodes(
        &mut self,
        new_groups: &HashMap<usize, Group>,
    ) -> Result<(), Box<dyn Error>> {
        for node in &mut self.nodes {
            match node {
                Node::GroupNode(group_node) => {
                    group_node.update_groups(new_groups)?;
                }
                Node::GroupVoicesNode(group_voices_node) => {
                    group_voices_node.update_groups(new_groups)?;
                }
                _ => (),
            }
        }
        Ok(())
    }

    pub(crate) fn set_note_on(&mut self, pitch: f32) {
        for node in &mut self.nodes {
            match node {
                Node::PitchNode(pitch_node) => {
                    pitch_node.set_pitch(pitch);
                }
                Node::GateNode(gate_node) => {
                    gate_node.set_is_active(true);
                }
                Node::GroupNode(group_node) => {
                    group_node.set_note_on(pitch);
                }
                Node::GroupVoicesNode(voice_group_node) => {
                    voice_group_node.set_note_on(pitch);
                }
                _ => (),
            }
        }
        self.last_pitch = pitch;
    }

    pub(crate) fn set_note_off(&mut self, pitch: f32) {
        if self.last_pitch != pitch {
            return;
        }
        for node in &mut self.nodes {
            match node {
                Node::GateNode(gate_node) => {
                    gate_node.set_is_active(false);
                }
                Node::GroupNode(group_node) => {
                    group_node.set_note_off(pitch);
                }
                Node::GroupVoicesNode(voice_group_node) => {
                    voice_group_node.set_note_off(pitch);
                }
                _ => (),
            }
        }
    }
}
