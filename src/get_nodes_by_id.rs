use crate::{node::Node, node_trait::NodeTrait, nodes_by_id::NodesById};
use std::collections::HashMap;

pub(crate) fn get_nodes_by_id(nodes: Vec<Node>) -> NodesById {
    let mut nodes_by_id: NodesById = HashMap::new();
    for node in nodes {
        nodes_by_id.insert(node.get_id(), node);
    }
    nodes_by_id
}
