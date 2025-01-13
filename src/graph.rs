use crate::{
    node::Node,
    node_trait::NodeTrait,
    node_values::NodeValues,
    nodes::{
        constant_node::ConstantNode, output_node::OutputNode, phase_node::PhaseNode,
        time_node::TimeNode, triangle_wave_node::TriangleWaveNode,
    },
};
use serde_json::Result;

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
    debug_string: String,
}

impl Graph {
    pub fn new() -> Self {
        let dev_nodes: Vec<Node> = vec![
            //
            Node::TimeNode(TimeNode::new(1)),
            Node::ConstantNode(ConstantNode::new(2, 440.)),
            Node::PhaseNode(PhaseNode::new(3, 1, 2)),
            Node::TriangleWaveNode(TriangleWaveNode::new(4, 3)),
            Node::OutputNode(OutputNode::new(5, 4)),
        ];
        Graph {
            nodes: dev_nodes,
            debug_string: "test1".to_owned(),
        }
    }

    pub fn set_nodes_from_json(&mut self, nodes_json: &str) -> Result<()> {
        self.nodes = serde_json::from_str(nodes_json)?;
        println!("Nodes: {:?}", self.nodes);
        Ok(())
    }

    pub fn get_debug_value(&self) -> f32 {
        42.
    }

    pub fn get_debug_string(&self) -> String {
        self.debug_string.clone()
    }

    pub fn set_debug_string(&mut self, value: &str) {
        println!("Received string: {}", value);
        self.debug_string = value.to_owned();
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
