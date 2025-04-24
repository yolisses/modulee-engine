#[macro_export]
macro_rules! declare_input_ids {
    ($($field:ident),+) => {
        use $crate::nodes::input_ids_trait::InputIdsTrait;
        use $crate::sort::node_indexes::NodeIndexes;

        #[derive(Debug, serde::Deserialize, Clone)]
        pub(crate) struct InputIds {
            $(pub $field: usize),+
        }

        impl InputIdsTrait for InputIds {
            fn set_from_node_indexes(&mut self, node_indexes: NodeIndexes) {
                $(self.$field = node_indexes[&self.$field];)+
            }
        }
    };
}
