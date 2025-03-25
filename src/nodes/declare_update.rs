#[macro_export]
macro_rules! declare_update {
    ($node_name:ident) => {
        use crate::has_update::HasUpdate;

        impl HasUpdate for $node_name {
            fn update(&mut self, new_node: &Self) {
                self.input_ids = new_node.input_ids.clone();
            }
        }
    };
}
