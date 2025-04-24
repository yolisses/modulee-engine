use crate::{
    get_inputs_trait::GetInputsTrait, has_update::HasUpdate,
    set_input_indexes_trait::SetInputIndexesTrait, sort::has_id::HasId,
};

pub(crate) trait NodeTrait:
    HasId + HasUpdate + GetInputsTrait + SetInputIndexesTrait
{
    fn process(&mut self, node_values: &[f32]) -> f32;

    fn get_is_pending(&self) -> bool {
        false
    }
}
