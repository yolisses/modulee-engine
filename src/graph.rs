use crate::{
    node::Node,
    node_trait::NodeTrait,
    node_values::NodeValues,
    sort::{has_id::HasId, sort_nodes_topologically::sort_nodes_topologically},
};
use serde_json::Result;

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new() -> Self {
        Graph { nodes: vec![] }
    }

    pub fn set_nodes_from_json(&mut self, nodes_json: &str) -> Result<()> {
        self.nodes = serde_json::from_str(nodes_json)?;
        sort_nodes_topologically(&mut self.nodes);
        println!("Nodes: {:?}", self.nodes);
        Ok(())
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
