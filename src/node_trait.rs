use crate::{has_inputs::HasInputs, has_update::HasUpdate, sort::has_id::HasId};

pub(crate) trait NodeTrait: HasId + HasInputs + HasUpdate {
    fn process(&mut self, node_values: &Vec<f32>) -> f32;

    fn get_is_pending(&self) -> bool {
        false
    }
}
