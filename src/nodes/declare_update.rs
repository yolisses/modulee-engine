#[macro_export]
macro_rules! declare_update {
    () => {
        fn update(&mut self, new_node: &Self) {
            self.input_ids = new_node.input_ids.clone();
        }
    };
}
