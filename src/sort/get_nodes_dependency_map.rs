use super::dependency_map::DependencyMap;
use crate::node_trait::NodeTrait;
use nohash_hasher::IntMap;

pub(crate) fn get_nodes_dependency_map(nodes: &Vec<impl NodeTrait>) -> DependencyMap {
    let mut inputs_mapping: DependencyMap = IntMap::default();

    for node in nodes {
        inputs_mapping.insert(node.get_id(), node.get_input_ids());
    }

    inputs_mapping
}
