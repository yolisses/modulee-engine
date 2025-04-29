use crate::{
    get_inputs_trait::GetInputsTrait,
    has_update::HasUpdate,
    node_trait::NodeTrait,
    nodes::{
        basic::{
            constant_node::ConstantNode, envelope_node::EnvelopeNode,
            frequency_node::FrequencyNode, gate_node::GateNode, phase_node::PhaseNode,
            pitch_node::PitchNode, time_node::TimeNode,
        },
        filter::{
            all_pass_node::AllPassNode, high_pass_node::HighPassNode, low_pass_node::LowPassNode,
            peak_node::PeakNode,
        },
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
    set_input_indexes_trait::SetInputIndexesTrait,
    set_note_trait::SetNoteTrait,
    sort::has_id::HasId,
    sort::node_indexes::NodeIndexes,
};
use serde::Deserialize;

macro_rules! define_node_enum {
    ($($variant:ident($type:ty)),+ $(,)?) => {
        #[derive(Debug, Deserialize, Clone)]
        #[serde(tag = "type")]
        pub(crate) enum Node {
            $($variant($type)),+
        }

        impl GetInputsTrait for Node {
            fn get_input_ids(&self) -> Vec<usize> {
                match self {
                    $(Node::$variant(node) => node.get_input_ids()),+
                }
            }
        }

        impl SetInputIndexesTrait for Node {
            fn set_input_indexes(&mut self, node_indexes: &NodeIndexes){
                match self {
                    $(Node::$variant(node) => node.set_input_indexes(node_indexes)),+
                }
            }
        }

        impl HasUpdate for Node {
            fn update(&mut self, new_node: &Self) {
                match (self, new_node) {
                    $((Node::$variant(node), Node::$variant(new_node)) => node.update(new_node)),+,
                    _ => panic!("Invalid node: {:#?}", new_node),
                }
            }
        }

        impl NodeTrait for Node {
            fn process(&mut self, node_values: &[f32]) -> f32 {
                match self {
                    $(Node::$variant(node) => node.process(node_values)),+
                }
            }


            fn get_is_pending(&self) -> bool {
                match self {
                    $(Node::$variant(node) => node.get_is_pending()),+
                }
            }
        }

        impl HasId for Node {
            fn get_id(&self) -> usize {
                match self {
                    $(Node::$variant(node) => node.get_id()),+
                }
            }
        }
    };
}

define_node_enum! {
    AddNode(AddNode),
    GateNode(GateNode),
    PeakNode(PeakNode),
    TimeNode(TimeNode),
    InputNode(InputNode),
    NoiseNode(NoiseNode),
    PhaseNode(PhaseNode),
    PitchNode(PitchNode),
    DivideNode(DivideNode),
    OutputNode(OutputNode),
    RandomNode(RandomNode),
    ModuleNode(ModuleNode),
    AllPassNode(AllPassNode),
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
