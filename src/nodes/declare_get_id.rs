#[macro_export]
macro_rules! declare_get_id {
    ($node_name:ident) => {
        use $crate::sort::has_id::HasId;

        impl HasId for $node_name {
            fn get_id(&self) -> usize {
                self.id
            }
        }
    };
}
