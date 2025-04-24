use super::{
    get_inputs_mapping::get_inputs_mapping,
    get_topologically_sorted_node_indexes::get_topologically_sorted_node_indexes,
    validate_inputs_mapping::validate_inputs_mapping,
};
use crate::node_trait::NodeTrait;

pub(crate) fn sort_nodes_topologically(nodes: &mut Vec<impl NodeTrait>) -> Result<(), String> {
    let inputs_mapping = get_inputs_mapping(nodes);
    validate_inputs_mapping(&inputs_mapping)?;
    let node_indexes = get_topologically_sorted_node_indexes(&inputs_mapping);
    nodes.sort_by_key(|node| node_indexes[&node.get_id()]);
    nodes
        .iter_mut()
        .for_each(|node| node.set_input_indexes(&node_indexes));
    Ok(())
}
