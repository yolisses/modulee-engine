use crate::{
    get_inputs_trait::GetInputsTrait,
    has_update::HasUpdate,
    node_trait::NodeTrait,
    nodes::{
        basic::{
            constant_node::ConstantNode, control_node::ControlNode, delay_node::DelayNode,
            envelope_node::EnvelopeNode, frequency_node::FrequencyNode, gate_node::GateNode,
            hold_node::HoldNode, phase_node::PhaseNode, pitch_node::PitchNode,
            sample_rate_node::SampleRateNode, time_node::TimeNode,
        },
        boolean::{and_node::AndNode, if_node::IfNode, not_node::NotNode, or_node::OrNode},
        filter::{
            all_pass_node::AllPassNode, high_pass_node::HighPassNode, low_pass_node::LowPassNode,
            peak_node::PeakNode,
        },
        math::{
            add_node::AddNode, ceil_node::CeilNode, cosine_node::CosineNode,
            divide_node::DivideNode, equals_node::EqualsNode, floor_node::FloorNode,
            greater_node::GreaterNode, modulo_node::ModuloNode, multiply_node::MultiplyNode,
            round_node::RoundNode, sine_node::SineNode, subtract_node::SubtractNode,
        },
        module::{
            input_node::InputNode, module_node::ModuleNode, module_voices_node::ModuleVoicesNode,
            output_node::OutputNode, value_from_channel_node::ValueFromChannelNode,
        },
        random::{random_from_value_node::RandomFromValueNode, random_node::RandomNode},
        wave::{
            noise_node::NoiseNode, pulse_wave_node::PulseWaveNode,
            sawtooth_wave_node::SawtoothWaveNode, sine_wave_node::SineWaveNode,
            triangle_wave_node::TriangleWaveNode,
        },
    },
    set_input_indexes_trait::SetInputIndexesTrait,
    set_note_trait::SetNoteTrait,
    set_sample_rate_trait::SetSampleRateTrait,
    sort::{has_id::HasId, node_indexes::NodeIndexes},
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
            fn process(&mut self, node_values: &[f32], external_node_values: &[f32]) -> f32 {
                match self {
                    $(Node::$variant(node) => node.process(node_values, external_node_values)),+
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
    AllPassNode(AllPassNode),
    AndNode(AndNode),
    CeilNode(CeilNode),
    ConstantNode(ConstantNode),
    ControlNode(ControlNode),
    CosineNode(CosineNode),
    DelayNode(DelayNode),
    DivideNode(DivideNode),
    EnvelopeNode(EnvelopeNode),
    EqualsNode(EqualsNode),
    FloorNode(FloorNode),
    FrequencyNode(FrequencyNode),
    GateNode(GateNode),
    GreaterNode(GreaterNode),
    HighPassNode(HighPassNode),
    HoldNode(HoldNode),
    IfNode(IfNode),
    InputNode(InputNode),
    LowPassNode(LowPassNode),
    ModuleNode(ModuleNode),
    ModuleVoicesNode(ModuleVoicesNode),
    ModuloNode(ModuloNode),
    MultiplyNode(MultiplyNode),
    NoiseNode(NoiseNode),
    NotNode(NotNode),
    OrNode(OrNode),
    OutputNode(OutputNode),
    PeakNode(PeakNode),
    PhaseNode(PhaseNode),
    PitchNode(PitchNode),
    PulseWaveNode(PulseWaveNode),
    RandomFromValueNode(RandomFromValueNode),
    RandomNode(RandomNode),
    RoundNode(RoundNode),
    SampleRateNode(SampleRateNode),
    SawtoothWaveNode(SawtoothWaveNode),
    SineNode(SineNode),
    SineWaveNode(SineWaveNode),
    SubtractNode(SubtractNode),
    TimeNode(TimeNode),
    TriangleWaveNode(TriangleWaveNode),
    ValueFromChannelNode(ValueFromChannelNode),
}

impl SetNoteTrait for Node {
    fn set_note_on(&mut self, pitch: f32) {
        match self {
            Node::EnvelopeNode(node) => node.set_note_on(pitch),
            Node::GateNode(node) => node.set_note_on(pitch),
            Node::ModuleNode(node) => node.set_note_on(pitch),
            Node::ModuleVoicesNode(node) => node.set_note_on(pitch),
            Node::PitchNode(node) => node.set_note_on(pitch),
            _ => (),
        }
    }

    fn set_note_off(&mut self, pitch: f32) {
        match self {
            Node::EnvelopeNode(node) => node.set_note_off(pitch),
            Node::GateNode(node) => node.set_note_off(pitch),
            Node::ModuleNode(node) => node.set_note_off(pitch),
            Node::ModuleVoicesNode(node) => node.set_note_off(pitch),
            Node::PitchNode(node) => node.set_note_off(pitch),
            _ => (),
        }
    }
}

impl SetSampleRateTrait for Node {
    fn set_sample_rate(&mut self, sample_rate: f32) {
        match self {
            Node::AllPassNode(node) => node.set_sample_rate(sample_rate),
            Node::ControlNode(node) => node.set_sample_rate(sample_rate),
            Node::DelayNode(node) => node.set_sample_rate(sample_rate),
            Node::EnvelopeNode(node) => node.set_sample_rate(sample_rate),
            Node::HighPassNode(node) => node.set_sample_rate(sample_rate),
            Node::LowPassNode(node) => node.set_sample_rate(sample_rate),
            Node::ModuleNode(node) => node.set_sample_rate(sample_rate),
            Node::ModuleVoicesNode(node) => node.set_sample_rate(sample_rate),
            Node::PeakNode(node) => node.set_sample_rate(sample_rate),
            Node::PhaseNode(node) => node.set_sample_rate(sample_rate),
            Node::SampleRateNode(node) => node.set_sample_rate(sample_rate),
            Node::TimeNode(node) => node.set_sample_rate(sample_rate),
            _ => (),
        }
    }
}
