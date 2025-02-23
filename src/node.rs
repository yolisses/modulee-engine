use crate::{
    node_trait::NodeTrait,
    nodes::{
        add_node::AddNode, constant_node::ConstantNode, divide_node::DivideNode,
        envelope_node::EnvelopeNode, frequency_node::FrequencyNode, gate_node::GateNode,
        group_node::GroupNode, group_voices_node::GroupVoicesNode, high_pass_node::HighPassNode,
        input_node::InputNode, multiply_node::MultiplyNode, noise_node::NoiseNode,
        output_node::OutputNode, phase_node::PhaseNode, pitch_node::PitchNode,
        sine_wave_node::SineWaveNode, subtract_node::SubtractNode, time_node::TimeNode,
        triangle_wave_node::TriangleWaveNode,
    },
    set_note_trait::SetNoteTrait,
    sort::has_id::HasId,
    values_by_id::ValuesById,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub(crate) enum Node {
    AddNode(AddNode),
    GateNode(GateNode),
    TimeNode(TimeNode),
    GroupNode(GroupNode),
    InputNode(InputNode),
    NoiseNode(NoiseNode),
    PhaseNode(PhaseNode),
    PitchNode(PitchNode),
    DivideNode(DivideNode),
    OutputNode(OutputNode),
    EnvelopeNode(EnvelopeNode),
    HighPassNode(HighPassNode),
    SubtractNode(SubtractNode),
    ConstantNode(ConstantNode),
    MultiplyNode(MultiplyNode),
    SineWaveNode(SineWaveNode),
    FrequencyNode(FrequencyNode),
    GroupVoicesNode(GroupVoicesNode),
    TriangleWaveNode(TriangleWaveNode),
}

impl NodeTrait for Node {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        match self {
            Node::AddNode(node) => node.process(node_values),
            Node::GateNode(node) => node.process(node_values),
            Node::TimeNode(node) => node.process(node_values),
            Node::PhaseNode(node) => node.process(node_values),
            Node::GroupNode(node) => node.process(node_values),
            Node::InputNode(node) => node.process(node_values),
            Node::NoiseNode(node) => node.process(node_values),
            Node::PitchNode(node) => node.process(node_values),
            Node::OutputNode(node) => node.process(node_values),
            Node::DivideNode(node) => node.process(node_values),
            Node::EnvelopeNode(node) => node.process(node_values),
            Node::HighPassNode(node) => node.process(node_values),
            Node::SubtractNode(node) => node.process(node_values),
            Node::ConstantNode(node) => node.process(node_values),
            Node::MultiplyNode(node) => node.process(node_values),
            Node::SineWaveNode(node) => node.process(node_values),
            Node::FrequencyNode(node) => node.process(node_values),
            Node::GroupVoicesNode(node) => node.process(node_values),
            Node::TriangleWaveNode(node) => node.process(node_values),
        }
    }

    fn get_input_ids(&self) -> Vec<usize> {
        match self {
            Node::AddNode(node) => node.get_input_ids(),
            Node::GateNode(node) => node.get_input_ids(),
            Node::TimeNode(node) => node.get_input_ids(),
            Node::GroupNode(node) => node.get_input_ids(),
            Node::InputNode(node) => node.get_input_ids(),
            Node::NoiseNode(node) => node.get_input_ids(),
            Node::PhaseNode(node) => node.get_input_ids(),
            Node::PitchNode(node) => node.get_input_ids(),
            Node::DivideNode(node) => node.get_input_ids(),
            Node::OutputNode(node) => node.get_input_ids(),
            Node::EnvelopeNode(node) => node.get_input_ids(),
            Node::HighPassNode(node) => node.get_input_ids(),
            Node::SubtractNode(node) => node.get_input_ids(),
            Node::ConstantNode(node) => node.get_input_ids(),
            Node::MultiplyNode(node) => node.get_input_ids(),
            Node::SineWaveNode(node) => node.get_input_ids(),
            Node::FrequencyNode(node) => node.get_input_ids(),
            Node::GroupVoicesNode(node) => node.get_input_ids(),
            Node::TriangleWaveNode(node) => node.get_input_ids(),
        }
    }

    fn update(&mut self, new_node: &Self) {
        match (self, new_node) {
            (Node::AddNode(node), Node::AddNode(new_node)) => node.update(new_node),
            (Node::GateNode(node), Node::GateNode(new_node)) => node.update(new_node),
            (Node::TimeNode(node), Node::TimeNode(new_node)) => node.update(new_node),
            (Node::GroupNode(node), Node::GroupNode(new_node)) => node.update(new_node),
            (Node::InputNode(node), Node::InputNode(new_node)) => node.update(new_node),
            (Node::NoiseNode(node), Node::NoiseNode(new_node)) => node.update(new_node),
            (Node::PhaseNode(node), Node::PhaseNode(new_node)) => node.update(new_node),
            (Node::PitchNode(node), Node::PitchNode(new_node)) => node.update(new_node),
            (Node::DivideNode(node), Node::DivideNode(new_node)) => node.update(new_node),
            (Node::OutputNode(node), Node::OutputNode(new_node)) => node.update(new_node),
            (Node::EnvelopeNode(node), Node::EnvelopeNode(new_node)) => node.update(new_node),
            (Node::HighPassNode(node), Node::HighPassNode(new_node)) => node.update(new_node),
            (Node::SubtractNode(node), Node::SubtractNode(new_node)) => node.update(new_node),
            (Node::ConstantNode(node), Node::ConstantNode(new_node)) => node.update(new_node),
            (Node::MultiplyNode(node), Node::MultiplyNode(new_node)) => node.update(new_node),
            (Node::SineWaveNode(node), Node::SineWaveNode(new_node)) => node.update(new_node),
            (Node::FrequencyNode(node), Node::FrequencyNode(new_node)) => node.update(new_node),
            (Node::GroupVoicesNode(node), Node::GroupVoicesNode(new_node)) => node.update(new_node),
            (Node::TriangleWaveNode(node), Node::TriangleWaveNode(new_node)) => {
                node.update(new_node)
            }
            _ => {
                panic!("Invalid node: {:#?}", new_node);
            }
        };
    }
}

impl HasId for Node {
    fn get_id(&self) -> usize {
        match self {
            Node::AddNode(node) => node.get_id(),
            Node::GateNode(node) => node.get_id(),
            Node::TimeNode(node) => node.get_id(),
            Node::GroupNode(node) => node.get_id(),
            Node::InputNode(node) => node.get_id(),
            Node::NoiseNode(node) => node.get_id(),
            Node::PhaseNode(node) => node.get_id(),
            Node::PitchNode(node) => node.get_id(),
            Node::OutputNode(node) => node.get_id(),
            Node::DivideNode(node) => node.get_id(),
            Node::EnvelopeNode(node) => node.get_id(),
            Node::HighPassNode(node) => node.get_id(),
            Node::SubtractNode(node) => node.get_id(),
            Node::ConstantNode(node) => node.get_id(),
            Node::MultiplyNode(node) => node.get_id(),
            Node::SineWaveNode(node) => node.get_id(),
            Node::FrequencyNode(node) => node.get_id(),
            Node::GroupVoicesNode(node) => node.get_id(),
            Node::TriangleWaveNode(node) => node.get_id(),
        }
    }
}

impl SetNoteTrait for Node {
    fn set_note_on(&mut self, pitch: f32) {
        match self {
            Node::GateNode(node) => node.set_note_on(pitch),
            Node::PitchNode(node) => node.set_note_on(pitch),
            Node::GroupNode(node) => node.set_note_on(pitch),
            Node::EnvelopeNode(node) => node.set_note_on(pitch),
            Node::GroupVoicesNode(node) => node.set_note_on(pitch),
            _ => (),
        }
    }

    fn set_note_off(&mut self, pitch: f32) {
        match self {
            Node::GateNode(node) => node.set_note_off(pitch),
            Node::PitchNode(node) => node.set_note_off(pitch),
            Node::GroupNode(node) => node.set_note_off(pitch),
            Node::EnvelopeNode(node) => node.set_note_off(pitch),
            Node::GroupVoicesNode(node) => node.set_note_off(pitch),
            _ => (),
        }
    }
}
