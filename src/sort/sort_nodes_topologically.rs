use super::{
    get_indexes_map::get_indexes_map, get_inputs_mapping::get_inputs_mapping,
    sort_topologically::sort_topologically, validate_inputs_mapping::validate_inputs_mapping,
};
use crate::node_trait::NodeTrait;

pub(crate) fn sort_nodes_topologically(nodes: &mut Vec<impl NodeTrait>) -> Result<(), String> {
    let inputs_mapping = get_inputs_mapping(nodes);
    validate_inputs_mapping(&inputs_mapping)?;
    let nodes_order = sort_topologically(&inputs_mapping);
    nodes.sort_by_key(|node| nodes_order.iter().position(|id| *id == node.get_id()));
    let node_indexes = get_indexes_map(nodes_order);
    nodes
        .iter_mut()
        .for_each(|node| node.set_input_indexes(&node_indexes));
    Ok(())
}
