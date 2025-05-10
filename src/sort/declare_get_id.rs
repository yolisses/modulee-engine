#[macro_export]
macro_rules! declare_get_id {
    ($struct_name:ident) => {
        use $crate::sort::has_id::HasId;

        impl HasId for $struct_name {
            fn get_id(&self) -> usize {
                self.id
            }
        }
    };
}
