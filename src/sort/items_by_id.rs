use super::has_id::HasId;
use std::collections::HashMap;

pub(crate) type ItemsById<T: HasId> = HashMap<usize, T>;
