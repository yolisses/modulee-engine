use crate::{
    node::Node,
    nodes::{
        constant_node::ConstantNode, output_node::OutputNode, phase_node::PhaseNode,
        time_node::TimeNode, triangle_wave_node::TriangleWaveNode,
    },
};

pub(crate) fn get_dev_nodes() -> Vec<Node> {
    vec![
        Node::TimeNode(TimeNode::new(1)),
        Node::ConstantNode(ConstantNode::new(2, 220.)),
        Node::PhaseNode(PhaseNode::new(3, 1, 2)),
        Node::TriangleWaveNode(TriangleWaveNode::new(4, 3)),
        Node::OutputNode(OutputNode::new(5, 4)),
    ]
}
