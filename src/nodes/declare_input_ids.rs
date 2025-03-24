// macros.rs
#[macro_export]
macro_rules! declare_input_ids {
    ($($field:ident),+) => {
        #[derive(Debug, serde::Deserialize, Clone)]
        pub(crate) struct InputIds {
            $(pub $field: usize),+
        }
    };
}
