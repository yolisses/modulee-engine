use crate::{declare_get_id, node::Node, node_trait::NodeTrait, set_note_trait::SetNoteTrait};
use nohash_hasher::IntMap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Module {
    id: usize,
    pub(crate) nodes: Vec<Node>,
    /// The output values of the nodes of the module. It's used only in the
    /// process method, but is declared in the struct to prevent costly
    /// allocations in each iteration.
    ///
    /// Also by performance reasons, it's a vector instead of a hash map.
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
    pub(crate) fn get_output_values(&self) -> (f32, f32) {
        let mut values = (0., 0.);
        for node in &self.nodes {
            if let Node::OutputNode(output_node) = node {
                match output_node.get_channel() {
                    0 => values.0 = output_node.get_value(),
                    1 => values.1 = output_node.get_value(),
                    _ => (),
                }
            }
        }
        values
    }

    pub(crate) fn process(&mut self) {
        self.update_value_from_channel_nodes();
        for (index, node) in self.nodes.iter_mut().enumerate() {
            let value = node.process(&self.node_values);
            self.node_values[index] = value;
        }
    }

    pub(crate) fn get_module_node_last_outputs(&self, input_id: usize) -> (f32, f32) {
        for node in &self.nodes {
            if node.get_id() == input_id {
                match node {
                    Node::ModuleNode(node) => return node.get_last_outputs(),
                    Node::ModuleVoicesNode(node) => return node.get_last_outputs(),
                    _ => continue,
                }
            }
        }
        (0.0, 0.0)
    }

    pub(crate) fn get_value_from_module_node(&self, input_id: usize, channel: u8) -> f32 {
        let last_outputs = self.get_module_node_last_outputs(input_id);
        match channel {
            0 => last_outputs.0,
            1 => last_outputs.1,
            _ => 0.,
        }
    }

    pub(crate) fn update_value_from_channel_nodes(&mut self) {
        for node in self.nodes.iter_mut() {
            if let Node::ValueFromChannelNode(value_from_channel_node) = node {
                let channel = value_from_channel_node.get_channel();
                let input_id = value_from_channel_node.get_input_id();
                let value = self.get_value_from_module_node(input_id, channel);
                value_from_channel_node.set_value(value);
            }
        }
    }
    pub(crate) fn get_is_pending(&self) -> bool {
        self.nodes.iter().any(|node| node.get_is_pending())
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

    pub(crate) fn get_child_module_ids(&self) -> Vec<usize> {
        let mut child_module_ids = vec![];

        for node in &self.nodes {
            match node {
                Node::ModuleNode(node) => {
                    if let Some(target_module_id) = node.get_target_module_id() {
                        child_module_ids.push(target_module_id);
                    }
                }
                Node::ModuleVoicesNode(node) => {
                    if let Some(target_module_id) = node.get_target_module_id() {
                        child_module_ids.push(target_module_id);
                    }
                }
                _ => (),
            }
        }

        child_module_ids
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
