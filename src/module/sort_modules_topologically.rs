use super::get_module_dependency_map::get_module_dependency_map;
use crate::module::module::Module;

pub(crate) fn sort_modules_topologically(modules: &mut Vec<Module>) -> Result<(), String> {
    let dependency_map = get_module_dependency_map(modules);
    validate_dependency_map(&dependency_map)?;
    let nodes_order = sort_topologically(&dependency_map);
    modules.sort_by_key(|node| nodes_order.iter().position(|id| *id == node.get_id()));
    let node_indexes = get_indexes_map(nodes_order);
    modules
        .iter_mut()
        .for_each(|node| node.set_input_indexes(&node_indexes));
    Ok(())
}
