#[macro_export]
macro_rules! declare_get_input_ids {
    ($node_name:ident, $($field:ident),+) => {
    use crate::has_inputs::HasInputs;

    impl HasInputs for $node_name {
        fn get_input_ids(&self) -> Vec<usize> {
            vec![
                $(self.input_ids.$field),+
            ]
        }
    }
    };
}
