use crate::{
    has_inputs::HasInputs, has_update::HasUpdate, sort::has_id::HasId, values_by_id::ValuesById,
};

// TODO move each responsibility to a different trait, allowing easier macro
// creation, and make NodeTrait simply a combination of them.
pub(crate) trait NodeTrait: HasId + HasInputs + HasUpdate {
    fn process(&mut self, node_values: &ValuesById) -> f32;

    fn get_is_pending(&self) -> bool {
        false
    }
}
