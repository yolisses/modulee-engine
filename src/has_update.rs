pub(crate) trait HasUpdate {
    fn update(&mut self, new_node: &Self);
}
