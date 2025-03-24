use crate::{
    node_trait::NodeTrait,
    nodes::{
        basic::{
            constant_node::ConstantNode, envelope_node::EnvelopeNode,
            frequency_node::FrequencyNode, gate_node::GateNode, phase_node::PhaseNode,
            pitch_node::PitchNode, time_node::TimeNode,
        },
        filter::{high_pass_node::HighPassNode, low_pass_node::LowPassNode},
        math::{
            add_node::AddNode, divide_node::DivideNode, multiply_node::MultiplyNode,
            subtract_node::SubtractNode,
        },
        module::{
            input_node::InputNode, module_node::ModuleNode, module_voices_node::ModuleVoicesNode,
            output_node::OutputNode,
        },
        random::{random_from_value_node::RandomFromValueNode, random_node::RandomNode},
        wave::{
            noise_node::NoiseNode, sawtooth_wave_node::SawtoothWaveNode,
            sine_wave_node::SineWaveNode, triangle_wave_node::TriangleWaveNode,
        },
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
    InputNode(InputNode),
    NoiseNode(NoiseNode),
    PhaseNode(PhaseNode),
    PitchNode(PitchNode),
    DivideNode(DivideNode),
    OutputNode(OutputNode),
    RandomNode(RandomNode),
    ModuleNode(ModuleNode),
    LowPassNode(LowPassNode),
    EnvelopeNode(EnvelopeNode),
    HighPassNode(HighPassNode),
    SubtractNode(SubtractNode),
    ConstantNode(ConstantNode),
    MultiplyNode(MultiplyNode),
    SineWaveNode(SineWaveNode),
    FrequencyNode(FrequencyNode),
    ModuleVoicesNode(ModuleVoicesNode),
    SawtoothWaveNode(SawtoothWaveNode),
    TriangleWaveNode(TriangleWaveNode),
    RandomFromValueNode(RandomFromValueNode),
}

impl NodeTrait for Node {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        match self {
            Node::AddNode(node) => node.process(node_values),
            Node::GateNode(node) => node.process(node_values),
            Node::TimeNode(node) => node.process(node_values),
            Node::PhaseNode(node) => node.process(node_values),
            Node::InputNode(node) => node.process(node_values),
            Node::NoiseNode(node) => node.process(node_values),
            Node::PitchNode(node) => node.process(node_values),
            Node::ModuleNode(node) => node.process(node_values),
            Node::DivideNode(node) => node.process(node_values),
            Node::OutputNode(node) => node.process(node_values),
            Node::RandomNode(node) => node.process(node_values),
            Node::LowPassNode(node) => node.process(node_values),
            Node::EnvelopeNode(node) => node.process(node_values),
            Node::HighPassNode(node) => node.process(node_values),
            Node::SubtractNode(node) => node.process(node_values),
            Node::ConstantNode(node) => node.process(node_values),
            Node::MultiplyNode(node) => node.process(node_values),
            Node::SineWaveNode(node) => node.process(node_values),
            Node::FrequencyNode(node) => node.process(node_values),
            Node::ModuleVoicesNode(node) => node.process(node_values),
            Node::SawtoothWaveNode(node) => node.process(node_values),
            Node::TriangleWaveNode(node) => node.process(node_values),
            Node::RandomFromValueNode(node) => node.process(node_values),
        }
    }

    fn get_input_ids(&self) -> Vec<usize> {
        match self {
            Node::AddNode(node) => node.get_input_ids(),
            Node::GateNode(node) => node.get_input_ids(),
            Node::TimeNode(node) => node.get_input_ids(),
            Node::InputNode(node) => node.get_input_ids(),
            Node::NoiseNode(node) => node.get_input_ids(),
            Node::PhaseNode(node) => node.get_input_ids(),
            Node::PitchNode(node) => node.get_input_ids(),
            Node::DivideNode(node) => node.get_input_ids(),
            Node::OutputNode(node) => node.get_input_ids(),
            Node::RandomNode(node) => node.get_input_ids(),
            Node::ModuleNode(node) => node.get_input_ids(),
            Node::LowPassNode(node) => node.get_input_ids(),
            Node::EnvelopeNode(node) => node.get_input_ids(),
            Node::HighPassNode(node) => node.get_input_ids(),
            Node::SubtractNode(node) => node.get_input_ids(),
            Node::ConstantNode(node) => node.get_input_ids(),
            Node::MultiplyNode(node) => node.get_input_ids(),
            Node::SineWaveNode(node) => node.get_input_ids(),
            Node::FrequencyNode(node) => node.get_input_ids(),
            Node::ModuleVoicesNode(node) => node.get_input_ids(),
            Node::SawtoothWaveNode(node) => node.get_input_ids(),
            Node::TriangleWaveNode(node) => node.get_input_ids(),
            Node::RandomFromValueNode(node) => node.get_input_ids(),
        }
    }

    fn update(&mut self, new_node: &Self) {
        match (self, new_node) {
            (Node::AddNode(node), Node::AddNode(new_node)) => node.update(new_node),
            (Node::GateNode(node), Node::GateNode(new_node)) => node.update(new_node),
            (Node::TimeNode(node), Node::TimeNode(new_node)) => node.update(new_node),
            (Node::InputNode(node), Node::InputNode(new_node)) => node.update(new_node),
            (Node::NoiseNode(node), Node::NoiseNode(new_node)) => node.update(new_node),
            (Node::PhaseNode(node), Node::PhaseNode(new_node)) => node.update(new_node),
            (Node::PitchNode(node), Node::PitchNode(new_node)) => node.update(new_node),
            (Node::ModuleNode(node), Node::ModuleNode(new_node)) => node.update(new_node),
            (Node::DivideNode(node), Node::DivideNode(new_node)) => node.update(new_node),
            (Node::OutputNode(node), Node::OutputNode(new_node)) => node.update(new_node),
            (Node::RandomNode(node), Node::RandomNode(new_node)) => node.update(new_node),
            (Node::LowPassNode(node), Node::LowPassNode(new_node)) => node.update(new_node),
            (Node::EnvelopeNode(node), Node::EnvelopeNode(new_node)) => node.update(new_node),
            (Node::HighPassNode(node), Node::HighPassNode(new_node)) => node.update(new_node),
            (Node::SubtractNode(node), Node::SubtractNode(new_node)) => node.update(new_node),
            (Node::ConstantNode(node), Node::ConstantNode(new_node)) => node.update(new_node),
            (Node::MultiplyNode(node), Node::MultiplyNode(new_node)) => node.update(new_node),
            (Node::SineWaveNode(node), Node::SineWaveNode(new_node)) => node.update(new_node),
            (Node::FrequencyNode(node), Node::FrequencyNode(new_node)) => node.update(new_node),
            (Node::ModuleVoicesNode(node), Node::ModuleVoicesNode(new_node)) => {
                node.update(new_node)
            }
            (Node::TriangleWaveNode(node), Node::TriangleWaveNode(new_node)) => {
                node.update(new_node)
            }
            (Node::SawtoothWaveNode(node), Node::SawtoothWaveNode(new_node)) => {
                node.update(new_node)
            }
            (Node::RandomFromValueNode(node), Node::RandomFromValueNode(new_node)) => {
                node.update(new_node)
            }
            _ => {
                panic!("Invalid node: {:#?}", new_node);
            }
        };
    }

    fn get_is_pending(&self) -> bool {
        match self {
            Node::AddNode(node) => node.get_is_pending(),
            Node::GateNode(node) => node.get_is_pending(),
            Node::TimeNode(node) => node.get_is_pending(),
            Node::InputNode(node) => node.get_is_pending(),
            Node::NoiseNode(node) => node.get_is_pending(),
            Node::PhaseNode(node) => node.get_is_pending(),
            Node::PitchNode(node) => node.get_is_pending(),
            Node::ModuleNode(node) => node.get_is_pending(),
            Node::DivideNode(node) => node.get_is_pending(),
            Node::OutputNode(node) => node.get_is_pending(),
            Node::RandomNode(node) => node.get_is_pending(),
            Node::LowPassNode(node) => node.get_is_pending(),
            Node::EnvelopeNode(node) => node.get_is_pending(),
            Node::HighPassNode(node) => node.get_is_pending(),
            Node::SubtractNode(node) => node.get_is_pending(),
            Node::ConstantNode(node) => node.get_is_pending(),
            Node::MultiplyNode(node) => node.get_is_pending(),
            Node::SineWaveNode(node) => node.get_is_pending(),
            Node::FrequencyNode(node) => node.get_is_pending(),
            Node::ModuleVoicesNode(node) => node.get_is_pending(),
            Node::SawtoothWaveNode(node) => node.get_is_pending(),
            Node::TriangleWaveNode(node) => node.get_is_pending(),
            Node::RandomFromValueNode(node) => node.get_is_pending(),
        }
    }
}

impl HasId for Node {
    fn get_id(&self) -> usize {
        match self {
            Node::AddNode(node) => node.get_id(),
            Node::GateNode(node) => node.get_id(),
            Node::TimeNode(node) => node.get_id(),
            Node::InputNode(node) => node.get_id(),
            Node::NoiseNode(node) => node.get_id(),
            Node::PhaseNode(node) => node.get_id(),
            Node::PitchNode(node) => node.get_id(),
            Node::DivideNode(node) => node.get_id(),
            Node::OutputNode(node) => node.get_id(),
            Node::RandomNode(node) => node.get_id(),
            Node::ModuleNode(node) => node.get_id(),
            Node::LowPassNode(node) => node.get_id(),
            Node::EnvelopeNode(node) => node.get_id(),
            Node::HighPassNode(node) => node.get_id(),
            Node::SubtractNode(node) => node.get_id(),
            Node::ConstantNode(node) => node.get_id(),
            Node::MultiplyNode(node) => node.get_id(),
            Node::SineWaveNode(node) => node.get_id(),
            Node::FrequencyNode(node) => node.get_id(),
            Node::ModuleVoicesNode(node) => node.get_id(),
            Node::TriangleWaveNode(node) => node.get_id(),
            Node::SawtoothWaveNode(node) => node.get_id(),
            Node::RandomFromValueNode(node) => node.get_id(),
        }
    }
}

impl SetNoteTrait for Node {
    fn set_note_on(&mut self, pitch: f32) {
        match self {
            Node::GateNode(node) => node.set_note_on(pitch),
            Node::PitchNode(node) => node.set_note_on(pitch),
            Node::ModuleNode(node) => node.set_note_on(pitch),
            Node::EnvelopeNode(node) => node.set_note_on(pitch),
            Node::ModuleVoicesNode(node) => node.set_note_on(pitch),
            _ => (),
        }
    }

    fn set_note_off(&mut self, pitch: f32) {
        match self {
            Node::GateNode(node) => node.set_note_off(pitch),
            Node::PitchNode(node) => node.set_note_off(pitch),
            Node::ModuleNode(node) => node.set_note_off(pitch),
            Node::EnvelopeNode(node) => node.set_note_off(pitch),
            Node::ModuleVoicesNode(node) => node.set_note_off(pitch),
            _ => (),
        }
    }
}
