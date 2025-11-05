use crate::{
    declare_get_id, node::Node, node_trait::NodeTrait,
    set_note_trait::SetNoteTrait, set_sample_rate_trait::SetSampleRateTrait,
};
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
    pub(crate) output_node_index: Option<usize>,
}

declare_get_id! {Module}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Module {
    pub(crate) fn get_output_value(&self) -> f32 {
        if let Some(output_node_index) = self.output_node_index {
            let node = &self.nodes[output_node_index];
            if let Node::OutputNode(node) = node {
                node.get_value()
            } else {
                0.
            }
        } else {
            0.
        }
    }

    pub(crate) fn process(&mut self, external_node_values: &[f32]) {
        for (index, node) in self.nodes.iter_mut().enumerate() {
            let value = node.process(&self.node_values, external_node_values);
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
