use super::dependency_map::DependencyMap;
use crate::node_trait::NodeTrait;
use vector_map::VecMap;

pub(crate) fn get_nodes_dependency_map(nodes: &Vec<impl NodeTrait>) -> DependencyMap {
    let mut dependency_map: DependencyMap = VecMap::default();

    for node in nodes {
        dependency_map.insert(node.get_id(), node.get_input_ids());
    }

    dependency_map
}
