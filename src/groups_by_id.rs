use crate::group::Group;
use nohash_hasher::IntMap;

pub(crate) type GroupsById = IntMap<usize, Group>;
