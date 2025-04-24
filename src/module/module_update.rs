use super::module::Module;
use crate::{
    has_update::HasUpdate,
    node::Node,
    sort::{has_id::HasId, sort_nodes_topologically::sort_nodes_topologically},
};
use std::error::Error;

impl Module {
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

        // Clears the node_values to prevent hard to find bugs involving deleted
        // nodes values
        self.node_values.clear();
        Ok(())
    }

    pub(crate) fn prepare_nodes(&mut self) {
        // TODO use result instead of unwrap
        sort_nodes_topologically(&mut self.nodes).unwrap();

        // TODO set input ids to indexes
    }

    pub(crate) fn prepare_modules_in_nodes(&mut self, possible_modules: &Vec<Module>) {
        for node in &mut self.nodes {
            match node {
                Node::ModuleNode(node) => node.prepare_module(possible_modules),
                Node::ModuleVoicesNode(node) => node.prepare_module(possible_modules),
                _ => (),
            }
        }
    }
}
