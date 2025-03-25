use crate::{declare_get_id, node_trait::NodeTrait, values_by_id::ValuesById};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct InputIds {
    input: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct OutputNode {
    id: usize,
    input_ids: InputIds,
    #[serde(skip)]
    value: f32,
}

declare_get_id! {OutputNode}

impl OutputNode {
    pub(crate) fn get_value(&self) -> f32 {
        self.value
    }
}

impl NodeTrait for OutputNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        let input = node_values[&self.input_ids.input];
        self.value = input;
        self.value
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![self.input_ids.input]
    }

    fn update(&mut self, new_node: &Self) {
        self.input_ids = new_node.input_ids.clone();
    }
}
