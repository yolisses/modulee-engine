use crate::{
    node::Node,
    node_trait::NodeTrait,
    set_note_trait::SetNoteTrait,
    sort::{has_id::HasId, sort_nodes_topologically::sort_nodes_topologically},
    values_by_id::ValuesById,
};
use nohash_hasher::IntMap;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct Module {
    id: usize,
    nodes: Vec<Node>,
    // node_values is a variable used in process method. It's here to prevent
    // costly allocations in each iteration
    #[serde(skip)]
    node_values: ValuesById,
}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl HasId for Module {
    fn get_id(&self) -> usize {
        self.id
    }
}

impl Module {
    // Passing node_values and input_target_ids may be a violation of the
    // responsibility division, but improves performance
    pub(crate) fn update_input_nodes(
        &mut self,
        node_values: &ValuesById,
        input_target_ids: &IntMap<usize, usize>,
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
            if let Node::OutputNode(output_node) = node {
                return output_node.get_value();
            }
        }
        0.
    }

    pub(crate) fn process(&mut self) {
        for node in &mut self.nodes {
            let value = node.process(&self.node_values);
            self.node_values.insert(node.get_id(), value);
        }
    }

    pub(crate) fn sort_nodes_topologically(&mut self) -> Result<(), Box<dyn Error>> {
        sort_nodes_topologically(&mut self.nodes)?;
        Ok(())
    }

    pub(crate) fn update(&mut self, new_module: &Module) -> Result<(), Box<dyn Error>> {
        let new_nodes = &new_module.nodes;

        // Remove nodes not present in new nodes
        self.nodes.retain(|node| {
            new_nodes
                .iter()
                .any(|new_node| new_node.get_id() == node.get_id())
        });

        for new_node in new_nodes {
            // Update a node if present in nodes. Saves a copy of the new node
            // otherwise
            let id = new_node.get_id();
            let node_option = self.nodes.iter_mut().find(|node| node.get_id() == id);
            if let Some(node) = node_option {
                node.update(new_node);
            } else {
                self.nodes.push(new_node.clone());
            }
        }

        self.sort_nodes_topologically()?;

        // Clears the node_values to prevent hard to find bugs involving deleted
        // nodes values
        self.node_values.clear();
        Ok(())
    }

    pub(crate) fn update_modules_in_nodes(
        &mut self,
        new_modules: &IntMap<usize, Module>,
    ) -> Result<(), Box<dyn Error>> {
        for node in &mut self.nodes {
            match node {
                Node::ModuleNode(module_node) => {
                    module_node.update_modules(new_modules)?;
                }
                Node::ModuleVoicesNode(module_voices_node) => {
                    module_voices_node.update_modules(new_modules)?;
                }
                _ => (),
            }
        }
        Ok(())
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
            match node {
                Node::ModuleVoicesNode(module_voices_node) => {
                    module_voices_node.remove_non_pending_voices();
                }
                _ => (),
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
