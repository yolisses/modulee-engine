use crate::{node_trait::NodeTrait, sort::has_id::HasId, values_by_id::ValuesById};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct InputIds {
    input: usize,
}

/// Saves the input value in a accessible way
#[derive(Debug, Deserialize)]
pub(crate) struct OutputNode {
    id: usize,
    input_ids: InputIds,
    #[serde(skip)]
    value: f32,
}

impl OutputNode {
    pub(crate) fn get_value(&self) -> f32 {
        self.value
    }
}

impl NodeTrait for OutputNode {
    fn process(&mut self, node_values: &mut ValuesById) {
        let input = node_values[&self.input_ids.input];
        self.value = input;
        let value = self.value;
        node_values.insert(self.id, value);
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
