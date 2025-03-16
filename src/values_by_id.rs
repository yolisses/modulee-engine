use crate::id::Id;
use nohash_hasher::IntMap;

pub(crate) type ValuesById = IntMap<Id, f32>;
