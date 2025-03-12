use crate::module::Module;
use nohash_hasher::IntMap;

pub(crate) type ModulesById = IntMap<usize, Module>;
