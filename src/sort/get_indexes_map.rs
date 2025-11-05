use super::node_indexes::NodeIndexes;
use crate::{node::Node, sort::has_id::HasId};

pub(crate) fn get_indexes_map(nodes: &Vec<Node>) -> NodeIndexes {
    let mut result: NodeIndexes = Default::default();
    for (index, node) in nodes.iter().enumerate() {
        result.insert(node.get_id(), index);
    }
    result
}
