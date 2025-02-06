use crate::{node_trait::NodeTrait, values_by_id::ValuesById, sort::has_id::HasId};
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct InputIds {
    input: usize,
}

/// Saves the input value in a accessible way
#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct OutputNode {
    #[serde(skip)]
    value: f32,
    id: usize,
    input_ids: InputIds,
}

impl OutputNode {
    pub(crate) fn get_value(&self) -> f32 {
        self.value
    }
}

impl NodeTrait for OutputNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        let input = node_values.get(&self.input_ids.input).unwrap();
        self.value = *input;
        self.value
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![self.input_ids.input]
    }
}

impl HasId for OutputNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
