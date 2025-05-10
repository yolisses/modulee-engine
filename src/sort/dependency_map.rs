use nohash_hasher::IntMap;

pub(crate) type DependencyMap = IntMap<usize, Vec<usize>>;
