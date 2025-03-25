#[macro_export]
macro_rules! declare_get_input_ids_and_its_getter {
    ($node_name:ident, $($field:ident),*) => {
        use $crate::declare_input_ids;
        use crate::declare_get_input_ids;

        declare_input_ids! {$($field),*}
        declare_get_input_ids! {$node_name, $($field),*}
    };
}
