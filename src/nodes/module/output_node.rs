use crate::{
    declare_get_id, declare_get_input_ids_and_its_getter, declare_update, node_trait::NodeTrait,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct OutputNode {
    id: usize,
    input_ids: InputIds,
    #[serde(skip)]
    value: f32,
}

declare_get_id! {OutputNode}
declare_update! {OutputNode}
declare_get_input_ids_and_its_getter! {OutputNode, input}

impl OutputNode {
    pub(crate) fn get_value(&self) -> f32 {
        self.value
    }
}

impl NodeTrait for OutputNode {
    fn process(&mut self, node_values: &[f32]) -> f32 {
        let input = node_values[self.input_ids.input];
        self.value = input;
        self.value
    }
}
