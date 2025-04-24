use nohash_hasher::IntMap;

/// Map where the key is the node id and the value is its index
pub(crate) type NodeIndexes = IntMap<usize, usize>;
