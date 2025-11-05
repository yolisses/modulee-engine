use crate::{
    declare_get_id, node::Node, node_trait::NodeTrait, set_note_trait::SetNoteTrait,
    set_sample_rate_trait::SetSampleRateTrait,
};
use serde::Deserialize;
use vector_map::VecMap;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Module {
    id: usize,
    pub(crate) nodes: Vec<Node>,

    /// The output values of the nodes of the module. It's used only in the
    /// process method, but is declared in the struct to prevent costly
    /// allocations in each iteration.
    ///
    /// Also by performance reasons, it's a vector instead of a hash map, which
    /// makes required using indexes instead of ids in node inputs.
    #[serde(skip)]
    pub(crate) node_values: Vec<f32>,

    /// Map where the key is the input node index and the value is the target
    /// node from an outer module index.
    #[serde(skip)]
    input_map: VecMap<usize, usize>,
}

declare_get_id! {Module}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Module {
    pub(crate) fn get_output_value(&self) -> f32 {
        for node in self.nodes.iter() {
            if let Node::OutputNode(node) = node {
                return node.get_value();
            };
        }
        0.
    }

    pub(crate) fn update_input_nodes_values(&mut self, node_values: &[f32]) {
        for (input_index, target_index) in &self.input_map {
            let node = &mut self.nodes[*input_index];
            if let Node::InputNode(input_node) = node {
                let value = node_values[*target_index];
                input_node.set_value(value);
            }
        }
    }

    pub(crate) fn process(&mut self) {
        for (index, node) in self.nodes.iter_mut().enumerate() {
            let value = node.process(&self.node_values);
            self.node_values[index] = value;
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

impl SetSampleRateTrait for Module {
    fn set_sample_rate(&mut self, sample_rate: f32) {
        for node in &mut self.nodes {
            node.set_sample_rate(sample_rate);
        }
    }
}
