use crate::{
    node::Node, node_trait::NodeTrait, node_values::NodeValues, nodes::time_node::TimeNode,
};

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new() -> Self {
        let dev_nodes: Vec<Node> = vec![
            //
            Node::TimeNode(TimeNode::new(1)),
        ];
        Graph { nodes: dev_nodes }
    }

    pub fn get_debug_value(&self) -> f64 {
        42.
    }

    pub fn get_output_value(&self) -> f64 {
        for node in &self.nodes {
            match node {
                Node::OutputNode(output_node) => return output_node.get_value(),
                _ => (),
            }
        }
        0.
    }

    pub fn process(&mut self) {
        let mut node_values = NodeValues::new();
        self.nodes.iter_mut().for_each(|node| {
            let output = node.process(&node_values);
            node_values.insert(node.get_id(), output);
        });
    }
}
