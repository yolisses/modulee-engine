#[macro_export]
macro_rules! declare_get_input_ids_and_its_getter {
    ($node_name:ident, $($field:ident),*) => {
        use $crate::declare_input_ids;

        declare_input_ids! {$($field),*}

        impl GetInputsTrait for $node_name {
            fn get_input_ids(&self) -> Vec<usize> {
                self.input_ids.get_input_ids()
            }
        }

        impl SetInputIndexesTrait for $node_name {
            fn set_input_indexes(&mut self, node_indexes: &NodeIndexes){
                self.input_ids.set_input_indexes(node_indexes);
            }
        }
    };
}
