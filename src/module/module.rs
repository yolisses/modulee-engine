use crate::{declare_get_id, node::Node, node_trait::NodeTrait, set_note_trait::SetNoteTrait};
use nohash_hasher::IntMap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Module {
    id: usize,
    pub(crate) nodes: Vec<Node>,
    // node_values is a variable used only in the process method. It is declared
    // in the struct to prevent costly allocations in each iteration
    #[serde(skip)]
    pub(crate) node_values: Vec<f32>,
}

declare_get_id! {Module}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Module {
    pub(crate) fn get_output_value(&self) -> f32 {
        for node in &self.nodes {
            if let Node::OutputNode(output_node) = node {
                return output_node.get_value();
            }
        }
        0.
    }

    pub(crate) fn process(&mut self) {
        for (index, node) in self.nodes.iter_mut().enumerate() {
            let value = node.process(&self.node_values);
            self.node_values[index] = value;
        }
    }

    pub(crate) fn get_is_pending(&self) -> bool {
        for node in &self.nodes {
            if node.get_is_pending() {
                return true;
            }
        }
        false
    }

    pub(crate) fn remove_non_pending_voices(&mut self) {
        for node in &mut self.nodes {
            if let Node::ModuleVoicesNode(module_voices_node) = node {
                module_voices_node.remove_non_pending_voices();
            }
        }
    }

    // Passing node_values and input_target_ids may be a violation of the
    // responsibility division, but improves performance
    pub(crate) fn set_input_node_values(
        &mut self,
        node_values: &[f32],
        input_target_ids: &IntMap<usize, usize>,
    ) {
        for node in &mut self.nodes {
            if let Node::InputNode(input_node) = node {
                let target_id = input_target_ids[&input_node.get_id()];
                let value = node_values[target_id];
                input_node.set_value(value);
            }
        }
    }
}

impl SetNoteTrait for Module {
    fn set_note_on(&mut self, pitch: f32) {
        for node in &mut self.nodes {
            node.set_note_on(pitch);
        }
    }

    fn set_note_off(&mut self, pitch: f32) {
        for node in &mut self.nodes {
            node.set_note_off(pitch);
        }
    }
}
