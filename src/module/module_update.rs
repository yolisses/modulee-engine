use super::{module::Module, sort_by_other_vec_order::sort_by_other_vec_order};
use crate::{
    control_update_data::ControlUpdateData,
    has_update::HasUpdate,
    node::Node,
    sort::{has_id::HasId, sort_nodes_topologically::sort_nodes_topologically},
};

impl Module {
    pub(crate) fn update(&mut self, new_module: &Module) {
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

        sort_by_other_vec_order(&mut self.nodes, new_nodes);
        self.reset_node_values();
    }

    fn reset_node_values(&mut self) {
        self.node_values = vec![0.; self.nodes.len()];
        self.module_node_outputs = vec![];
    }

    pub(crate) fn prepare_nodes(&mut self) {
        // TODO use result instead of unwrap
        sort_nodes_topologically(&mut self.nodes).unwrap();
        self.reset_node_values();
    }

    pub(crate) fn prepare_modules_in_nodes(&mut self, possible_modules: &[Module]) {
        for node in &mut self.nodes {
            match node {
                Node::ModuleNode(node) => node.prepare_module(possible_modules),
                Node::ModuleVoicesNode(node) => node.prepare_module(possible_modules),
                _ => (),
            }
        }
    }

    pub(crate) fn update_control(&mut self, control_update_data: &ControlUpdateData) {
        for node in &mut self.nodes {
            match node {
                Node::ControlNode(node) => node.update_control(control_update_data),
                Node::ModuleNode(node) => node.update_control(control_update_data),
                Node::ModuleVoicesNode(node) => node.update_control(control_update_data),
                _ => (),
            }
        }
    }
}
