use vector_map::VecMap;

/// Map where the key is the node id and the value is its index
pub(crate) type NodeIndexes = VecMap<usize, usize>;
