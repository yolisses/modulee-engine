use crate::{
    node_trait::NodeTrait,
    node_values::NodeValues,
    nodes::{
        add_node::AddNode, constant_node::ConstantNode, divide_node::DivideNode,
        multiply_node::MultiplyNode, output_node::OutputNode, phase_node::PhaseNode,
        sine_wave_node::SineWaveNode, subtract_node::SubtractNode, time_node::TimeNode,
        triangle_wave_node::TriangleWaveNode,
    },
    sort::has_id::HasId,
};
use serde::Deserialize;

#[serde(tag = "type")]
#[derive(Debug, PartialEq, Deserialize)]
pub(crate) enum Node {
    AddNode(AddNode),
    TimeNode(TimeNode),
    PhaseNode(PhaseNode),
    DivideNode(DivideNode),
    OutputNode(OutputNode),
    SubtractNode(SubtractNode),
    ConstantNode(ConstantNode),
    MultiplyNode(MultiplyNode),
    SineWaveNode(SineWaveNode),
    TriangleWaveNode(TriangleWaveNode),
}

impl NodeTrait for Node {
    fn process(&mut self, node_values: &NodeValues) -> f32 {
        match self {
            Node::AddNode(add_node) => add_node.process(node_values),
            Node::TimeNode(time_node) => time_node.process(node_values),
            Node::PhaseNode(phase_node) => phase_node.process(node_values),
            Node::OutputNode(output_node) => output_node.process(node_values),
            Node::DivideNode(divide_node) => divide_node.process(node_values),
            Node::SubtractNode(subtract_node) => subtract_node.process(node_values),
            Node::ConstantNode(constant_node) => constant_node.process(node_values),
            Node::MultiplyNode(multiply_node) => multiply_node.process(node_values),
            Node::SineWaveNode(sine_wave_node) => sine_wave_node.process(node_values),
            Node::TriangleWaveNode(triangle_wave_node) => triangle_wave_node.process(node_values),
        }
    }

    fn get_input_ids(&self) -> Vec<usize> {
        match self {
            Node::AddNode(add_node) => add_node.get_input_ids(),
            Node::TimeNode(time_node) => time_node.get_input_ids(),
            Node::PhaseNode(phase_node) => phase_node.get_input_ids(),
            Node::DivideNode(divide_node) => divide_node.get_input_ids(),
            Node::OutputNode(output_node) => output_node.get_input_ids(),
            Node::SubtractNode(subtract_node) => subtract_node.get_input_ids(),
            Node::ConstantNode(constant_node) => constant_node.get_input_ids(),
            Node::MultiplyNode(multiply_node) => multiply_node.get_input_ids(),
            Node::SineWaveNode(sine_wave_node) => sine_wave_node.get_input_ids(),
            Node::TriangleWaveNode(triangle_wave_node) => triangle_wave_node.get_input_ids(),
        }
    }
}

impl HasId for Node {
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
            Node::SineWaveNode(sine_wave_node) => sine_wave_node.get_id(),
            Node::TriangleWaveNode(triangle_wave_node) => triangle_wave_node.get_id(),
        }
    }
}
