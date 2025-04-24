#[macro_export]
macro_rules! declare_empty_get_input_ids {
    ($node_name:ident) => {
        use $crate::get_inputs_trait::GetInputsTrait;
        use $crate::set_input_indexes_trait::SetInputIndexesTrait;
        use $crate::sort::node_indexes::NodeIndexes;

        impl GetInputsTrait for $node_name {
            fn get_input_ids(&self) -> Vec<usize> {
                vec![]
            }
        }

        impl SetInputIndexesTrait for $node_name {
            fn set_input_indexes(&mut self, _node_indexes: &NodeIndexes) {}
        }
    };
}
