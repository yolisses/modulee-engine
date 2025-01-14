use crate::{node_values::NodeValues, sort::has_id::HasId};

pub(crate) trait NodeTrait: HasId {
    fn get_input_ids(&self) -> Vec<usize>;
    fn process(&mut self, node_values: &NodeValues) -> f32;
}
