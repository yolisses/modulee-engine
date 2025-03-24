#[macro_export]
macro_rules! declare_get_id {
    () => {
        impl HasId for AddNode {
            fn get_id(&self) -> usize {
                self.id
            }
        }
    };
}
