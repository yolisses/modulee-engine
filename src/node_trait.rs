use crate::{sort::has_id::HasId, values_by_id::ValuesById};
pub(crate) trait NodeTrait: HasId {
    fn get_input_ids(&self) -> Vec<usize>;
    fn process(&mut self, node_values: &mut ValuesById);
}
