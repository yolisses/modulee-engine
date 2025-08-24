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
    #[serde(skip)]
    pub(crate) module_node_outputs: Vec<(usize, (f32, f32))>,
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
        self.module_node_outputs.clear(); // Clear Vec instead of creating new
        for (index, node) in self.nodes.iter_mut().enumerate() {
            Module::update_value_from_channel_node(node, &mut self.module_node_outputs);

            let value = node.process(&self.node_values);
            self.node_values[index] = value;

            Module::update_module_nodes_output(node, &mut self.module_node_outputs, index);
        }
    }

    pub(crate) fn update_value_from_channel_node(
        node: &mut Node,
        module_node_outputs: &mut Vec<(usize, (f32, f32))>,
    ) {
        if let Node::ValueFromChannelNode(value_from_channel_node) = node {
            let input_id = value_from_channel_node.get_input_id();
            let outputs = module_node_outputs
                .iter()
                .find(|(id, _)| *id == input_id)
                .map(|(_, v)| v);
            if let Some(outputs) = outputs {
                let channel = value_from_channel_node.get_channel();
                value_from_channel_node.set_value(match channel {
                    0 => outputs.0,
                    1 => outputs.1,
                    _ => 0.,
                });
            } else {
                value_from_channel_node.set_value(0.);
            }
        }
    }

    pub(crate) fn update_module_nodes_output(
        node: &Node,
        module_node_outputs: &mut Vec<(usize, (f32, f32))>,
        index: usize,
    ) {
        match node {
            Node::ModuleNode(node) => {
                module_node_outputs.push((index, node.get_last_outputs()));
            }
            Node::ModuleVoicesNode(node) => {
                module_node_outputs.push((index, node.get_last_outputs()));
            }
            _ => (),
        };
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
