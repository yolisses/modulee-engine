pub(crate) trait HasInputs {
    fn get_input_ids(&self) -> Vec<usize>;
}
