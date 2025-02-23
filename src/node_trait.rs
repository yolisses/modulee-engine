use crate::{sort::has_id::HasId, values_by_id::ValuesById};
pub(crate) trait NodeTrait: HasId {
    fn get_input_ids(&self) -> Vec<usize>;

    fn update(&mut self, new_node: &Self);

    fn process(&mut self, node_values: &ValuesById) -> f32;

    fn get_is_pending(&self) -> bool {
        false
    }
}
