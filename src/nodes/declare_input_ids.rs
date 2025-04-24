#[macro_export]
macro_rules! declare_input_ids {
    ($($field:ident),+) => {
        use $crate::sort::node_indexes::NodeIndexes;
        use $crate::get_inputs_trait::GetInputsTrait;
        use $crate::set_input_indexes_trait::SetInputIndexesTrait;

        #[derive(Debug, serde::Deserialize, Clone)]
        pub(crate) struct InputIds {
            $(pub $field: usize),+
        }

        impl GetInputsTrait for InputIds {
            fn get_input_ids(&self) -> Vec<usize> {
                vec![$(self.$field),*]
            }
        }

        impl SetInputIndexesTrait for InputIds {
            fn set_input_indexes(&mut self, node_indexes: &NodeIndexes) {
                $(self.$field = node_indexes[&self.$field];)+
            }
        }
    };
}
