use vector_map::VecMap;

use super::{module::Module, sort_by_other_vec_order::sort_by_other_vec_order};
use crate::{
    control_update_data::ControlUpdateData,
    has_update::HasUpdate,
    node::Node,
    set_input_indexes_trait::SetInputIndexesTrait,
    set_sample_rate_trait::SetSampleRateTrait,
    sort::{
        get_indexes_map::get_indexes_map, has_id::HasId, node_indexes::NodeIndexes,
        sort_nodes_topologically::sort_nodes_topologically,
    },
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

    // TODO check if it still makes sense.
    pub(crate) fn reset_node_values(&mut self) {
        self.node_values = vec![
            0.;
            self.nodes
                .iter()
                .map(|node| -> usize {
                    if let Node::ModuleNode(_) | Node::ModuleVoicesNode(_) = node {
                        2
                    } else {
                        1
                    }
                })
                .sum()
        ];
    }

    pub(crate) fn set_sample_rate(&mut self, sample_rate: f32) {
        for node in &mut self.nodes {
            node.set_sample_rate(sample_rate);
        }
    }

    pub(crate) fn sort_nodes_topologically(&mut self) -> Result<(), String> {
        // Put the output nodes first to leverage locality
        self.nodes
            .sort_by_key(|node| if let Node::OutputNode(_) = node { 0 } else { 1 });
        sort_nodes_topologically(&mut self.nodes)
    }

    pub(crate) fn set_node_ids_to_indexes(
        &mut self,
        outer_node_indexes: &NodeIndexes,
        input_target_ids: &VecMap<usize, usize>,
    ) {
        let inner_node_indexes = get_indexes_map(&self.nodes);
        self.update_input_map(outer_node_indexes, input_target_ids, &inner_node_indexes);

        for node in &mut self.nodes {
            node.set_input_indexes(&inner_node_indexes);
        }
    }

    fn update_input_map(
        &mut self,
        outer_node_indexes: &VecMap<usize, usize>,
        input_target_ids: &VecMap<usize, usize>,
        inner_node_indexes: &VecMap<usize, usize>,
    ) {
        self.input_map = Default::default();
        for (input_id, target_id) in input_target_ids {
            let input_index = inner_node_indexes.get(input_id);
            let target_index = outer_node_indexes.get(target_id);
            if let (Some(input_index), Some(target_index)) = (input_index, target_index) {
                self.input_map.insert(*input_index, *target_index);
            };
        }
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
