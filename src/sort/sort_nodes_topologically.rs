use super::{
    get_inputs_mapping::get_inputs_mapping,
    get_topologically_sorted_node_indexes::get_topologically_sorted_node_indexes, has_id::HasId,
};
use crate::node::Node;

pub(crate) fn sort_nodes_topologically(nodes: &mut Vec<Node>) {
    let inputs_mapping = get_inputs_mapping(nodes);
    let topologically_sorted_node_indexes = get_topologically_sorted_node_indexes(&inputs_mapping);
    nodes.sort_by_key(|node| topologically_sorted_node_indexes[&node.get_id()]);
}
