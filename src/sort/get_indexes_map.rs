use super::node_indexes::NodeIndexes;
use crate::{node::Node, sort::has_id::HasId};

pub(crate) fn get_indexes_map(nodes: &Vec<Node>) -> NodeIndexes {
    let mut result: NodeIndexes = Default::default();
    let mut index = 0;
    for node in nodes {
        result.insert(node.get_id(), index);
        if let Node::ModuleNode(_) | Node::ModuleVoicesNode(_) = node {
            index += 2;
        } else {
            index += 1;
        }
    }
    result
}
