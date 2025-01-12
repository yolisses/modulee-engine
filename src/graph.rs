use crate::{
    node::Node,
    node_trait::NodeTrait,
    node_values::NodeValues,
    nodes::{
        constant_node::ConstantNode, output_node::OutputNode, phase_node::PhaseNode,
        time_node::TimeNode,
    },
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
            Node::ConstantNode(ConstantNode::new(2, 440.)),
            Node::PhaseNode(PhaseNode::new(3, 1, 2)),
            Node::OutputNode(OutputNode::new(4, 3)),
        ];
        Graph { nodes: dev_nodes }
    }

    pub fn get_debug_value(&self) -> f32 {
        42.
    }

    pub fn get_output_value(&self) -> f32 {
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

    pub fn process_block(&mut self, buffer: &mut [f32], length: usize) {
        for index in 0..length {
            self.process();
            buffer[index] = self.get_output_value();
        }
    }
}
