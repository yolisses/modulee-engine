use crate::sort::node_indexes::NodeIndexes;

pub(crate) trait SetInputIndexesTrait {
    fn set_input_indexes(&mut self, node_indexes: &NodeIndexes);
}
