use crate::{node::Node, node_trait::NodeTrait, node_values::NodeValues};

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new() -> Self {
        Graph { nodes: Vec::new() }
    }

    pub fn set_nodes(&mut self, nodes_data: JsValue) -> Result<(), Error> {
        match serde_wasm_bindgen::from_value(nodes_data) {
            Ok(nodes) => {
                self.nodes = nodes;
                Ok(())
            }
            Err(_) => Err(Error::new("Invalid nodes data".to_owned())),
        }
    }

    pub fn process(&mut self) -> f64 {
        let mut node_values = NodeValues::new();
        self.nodes.iter_mut().for_each(|node| {
            let output = node.process(&node_values);
            node_values.insert(node.get_id(), output);
        });

        // TODO remove this hard coded value by a OutputNode last value
        let output_node_id = 3;
        node_values[&output_node_id]
    }
}
