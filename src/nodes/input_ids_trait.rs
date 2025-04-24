use crate::sort::node_indexes::NodeIndexes;

pub(crate) trait InputIdsTrait {
    fn set_from_node_indexes(&mut self, node_indexes: NodeIndexes);
}
