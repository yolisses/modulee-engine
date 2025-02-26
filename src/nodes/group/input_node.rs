use crate::{node_trait::NodeTrait, sort::has_id::HasId, values_by_id::ValuesById};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct InputNode {
    id: usize,
    #[serde(skip)]
    value: f32,
}

impl InputNode {
    pub(crate) fn set_value(&mut self, value: f32) {
        self.value = value;
    }
}

impl NodeTrait for InputNode {
    fn process(&mut self, _node_values: &ValuesById) -> f32 {
        self.value
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![]
    }

    fn update(&mut self, _new_node: &Self) {}
}

impl HasId for InputNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
