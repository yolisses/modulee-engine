#[macro_export]
macro_rules! declare_empty_get_input_ids {
    ($node_name:ident) => {
        use $crate::has_inputs::HasInputs;

        impl HasInputs for $node_name {
            fn get_input_ids(&self) -> Vec<usize> {
                vec![]
            }
        }
    };
}
