use super::inputs_mapping::InputsMapping;
use crate::node_trait::NodeTrait;
use nohash_hasher::IntMap;

pub(crate) fn get_inputs_mapping(nodes: &Vec<impl NodeTrait>) -> InputsMapping {
    let mut inputs_mapping: InputsMapping = IntMap::default();

    for node in nodes {
        inputs_mapping.insert(node.get_id(), node.get_input_ids());
    }

    inputs_mapping
}
