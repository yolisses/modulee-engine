use crate::module::module::Module;

pub(crate) fn sort_modules_topologically(nodes: &mut Vec<Module>) -> Result<(), String> {
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
