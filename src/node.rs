use crate::{
    node_trait::NodeTrait,
    node_values::NodeValues,
    nodes::{
        add_node::AddNode, constant_node::ConstantNode, divide_node::DivideNode,
        multiply_node::MultiplyNode, output_node::OutputNode, phase_node::PhaseNode,
        subtract_node::SubtractNode, time_node::TimeNode,
    },
};

#[derive(Debug)]
pub(crate) enum Node {
    AddNode(AddNode),
    TimeNode(TimeNode),
    PhaseNode(PhaseNode),
    DivideNode(DivideNode),
    OutputNode(OutputNode),
    SubtractNode(SubtractNode),
    ConstantNode(ConstantNode),
    MultiplyNode(MultiplyNode),
}

impl NodeTrait for Node {
    fn process(&mut self, node_values: &NodeValues) -> f64 {
        match self {
            Node::AddNode(add_node) => add_node.process(node_values),
            Node::TimeNode(time_node) => time_node.process(node_values),
            Node::PhaseNode(phase_node) => phase_node.process(node_values),
            Node::OutputNode(output_node) => output_node.process(node_values),
            Node::DivideNode(divide_node) => divide_node.process(node_values),
            Node::SubtractNode(subtract_node) => subtract_node.process(node_values),
            Node::ConstantNode(constant_node) => constant_node.process(node_values),
            Node::MultiplyNode(multiply_node) => multiply_node.process(node_values),
        }
    }

    fn get_id(&self) -> usize {
        match self {
            Node::AddNode(add_node) => add_node.get_id(),
            Node::TimeNode(time_node) => time_node.get_id(),
            Node::PhaseNode(phase_node) => phase_node.get_id(),
            Node::OutputNode(output_node) => output_node.get_id(),
            Node::DivideNode(divide_node) => divide_node.get_id(),
            Node::SubtractNode(subtract_node) => subtract_node.get_id(),
            Node::ConstantNode(constant_node) => constant_node.get_id(),
            Node::MultiplyNode(multiply_node) => multiply_node.get_id(),
        }
    }
}
