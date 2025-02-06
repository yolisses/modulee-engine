use super::{
    get_inputs_mapping::get_inputs_mapping,
    get_topologically_sorted_node_indexes::get_topologically_sorted_node_indexes, has_id::HasId,
    validate_inputs_mapping::validate_inputs_mapping,
};
use crate::node::Node;

pub(crate) fn sort_nodes_topologically(nodes: &mut Vec<Node>) -> Result<(), String> {
    let inputs_mapping = get_inputs_mapping(nodes);
    validate_inputs_mapping(&inputs_mapping)?;
    let topologically_sorted_node_indexes = get_topologically_sorted_node_indexes(&inputs_mapping);
    nodes.sort_by_key(|node| topologically_sorted_node_indexes[&node.get_id()]);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::nodes::{add_node::AddNode, constant_node::ConstantNode, output_node::OutputNode};

    use super::*;

    #[test]
    fn test_sort_nodes_topologically() {
        let mut nodes = vec![
            Node::ConstantNode(ConstantNode::new(0, 0.0)),
            Node::ConstantNode(ConstantNode::new(795039224, 42.0)),
            Node::OutputNode(OutputNode::new(805174500, 904106764)),
            Node::AddNode(AddNode::new(904106764, 0, 0)),
        ];

        sort_nodes_topologically(&mut nodes);

        assert_eq!(
            nodes,
            vec![
                Node::ConstantNode(ConstantNode::new(0, 0.0)),
                Node::ConstantNode(ConstantNode::new(795039224, 42.0)),
                Node::AddNode(AddNode::new(904106764, 0, 0)),
                Node::OutputNode(OutputNode::new(805174500, 904106764)),
            ]
        );
    }
}
