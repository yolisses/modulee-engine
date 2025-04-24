use crate::{get_inputs_trait::GetInputsTrait, set_input_indexes_trait::SetInputIndexesTrait};

pub(crate) trait InputIdsTrait: GetInputsTrait + SetInputIndexesTrait {}
