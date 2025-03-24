// macros.rs
#[macro_export]
macro_rules! declare_get_input_ids {
    ($($field:ident),+) => {
        fn get_input_ids(&self) -> Vec<usize> {
            vec![
                $(self.input_ids.$field),+
            ]
        }
    };
}
