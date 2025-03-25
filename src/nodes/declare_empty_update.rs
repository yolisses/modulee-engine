#[macro_export]
macro_rules! declare_empty_update {
    ($node_name:ident) => {
        use $crate::has_update::HasUpdate;

        impl HasUpdate for $node_name {
            fn update(&mut self, _new_node: &Self) {}
        }
    };
}
