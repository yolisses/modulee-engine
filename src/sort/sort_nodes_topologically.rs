use super::{
    get_nodes_dependency_map::get_nodes_dependency_map, sort_topologically::sort_topologically,
    validate_dependency_map::validate_dependency_map,
};
use crate::node_trait::NodeTrait;

pub(crate) fn sort_nodes_topologically(nodes: &mut Vec<impl NodeTrait>) -> Result<(), String> {
    let dependency_map = get_nodes_dependency_map(nodes);
    validate_dependency_map(&dependency_map)?;
    let nodes_order = sort_topologically(&dependency_map);
    nodes.sort_by_key(|node| nodes_order.iter().position(|id| *id == node.get_id()));
    Ok(())
}
