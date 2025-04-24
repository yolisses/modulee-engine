use crate::sort::node_indexes::NodeIndexes;

pub(crate) fn create_nodes_indexes(data: &[(usize, usize)]) -> NodeIndexes {
    let mut node_indexes: NodeIndexes = Default::default();
    for (key, value) in data {
        node_indexes.insert(*key, *value);
    }
    node_indexes
}
