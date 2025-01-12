use crate::{
    node_trait::NodeTrait,
    node_values::NodeValues,
    nodes::{
        add_node::AddNode, constant_node::ConstantNode, output_node::OutputNode,
        subtract_node::SubtractNode,
    },
};

#[derive(Debug)]
pub(crate) enum Node {
    AddNode(AddNode),
    OutputNode(OutputNode),
    SubtractNode(SubtractNode),
    ConstantNode(ConstantNode),
}

impl NodeTrait for Node {
    fn process(&mut self, node_values: &NodeValues) -> f64 {
        match self {
            Node::AddNode(add_node) => add_node.process(node_values),
            Node::OutputNode(output_node) => output_node.process(node_values),
            Node::SubtractNode(subtract_node) => subtract_node.process(node_values),
            Node::ConstantNode(constant_node) => constant_node.process(node_values),
        }
    }

    fn get_id(&self) -> usize {
        match self {
            Node::AddNode(add_node) => add_node.get_id(),
            Node::OutputNode(output_node) => output_node.get_id(),
            Node::SubtractNode(subtract_node) => subtract_node.get_id(),
            Node::ConstantNode(constant_node) => constant_node.get_id(),
        }
    }
}
