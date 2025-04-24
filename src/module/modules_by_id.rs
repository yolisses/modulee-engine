use super::module::Module;
use nohash_hasher::IntMap;

pub(crate) type ModulesById = IntMap<usize, Module>;
