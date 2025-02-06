use crate::{
    node::Node,
    node_trait::NodeTrait,
    values_by_id::ValuesById,
    sort::{has_id::HasId, sort_nodes_topologically::sort_nodes_topologically},
};
use serde_json::Result;

#[derive(Debug)]
pub struct Group {
    last_pitch: f32,
    nodes: Vec<Node>,
}

// TODO make polyphonic
impl Group {
    pub fn new() -> Self {
        Group {
            nodes: vec![],
            last_pitch: 0.,
        }
    }

    pub fn set_nodes_from_json(&mut self, nodes_json: &str) -> Result<()> {
        self.nodes = serde_json::from_str(nodes_json)?;
        sort_nodes_topologically(&mut self.nodes);
        println!("Nodes: {:?}", self.nodes);
        Ok(())
    }

    /// This is for development. It will be replaced by set_note_on
    pub fn set_pitch(&mut self, pitch: f32) {
        for node in &mut self.nodes {
            if let Node::PitchNode(pitch_node) = node {
                pitch_node.set_pitch(pitch);
            }
        }
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
        let mut node_values = ValuesById::new();
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

    pub fn set_note_on(&mut self, pitch: f32) {
        for node in &mut self.nodes {
            match node {
                Node::PitchNode(pitch_node) => {
                    pitch_node.set_pitch(pitch);
                }
                Node::GateNode(gate_node) => {
                    gate_node.set_is_active(true);
                }
                _ => (),
            }
        }
        self.last_pitch = pitch;
    }

    pub fn set_note_off(&mut self, pitch: f32) {
        if self.last_pitch != pitch {
            return;
        }
        for node in &mut self.nodes {
            if let Node::GateNode(gate_node) = node {
                gate_node.set_is_active(false);
            }
        }
    }
}
