use crate::{
    node_trait::NodeTrait, sample_rate::SAMPLE_RATE, sort::has_id::HasId, values_by_id::ValuesById,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct InputIds {}

/// Returns the current time in seconds
#[derive(Debug, Deserialize)]
pub(crate) struct TimeNode {
    #[serde(skip)]
    value: f32,
    id: usize,
    input_ids: InputIds,
}

impl NodeTrait for TimeNode {
    fn process(&mut self, node_values: &mut ValuesById) {
        self.value += 1. / SAMPLE_RATE;
        let value = self.value;
        node_values.insert(self.id, value);
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![]
    }
}

impl HasId for TimeNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
