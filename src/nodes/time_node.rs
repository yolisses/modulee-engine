use crate::{
    node_trait::NodeTrait, sample_rate::SAMPLE_RATE, sort::has_id::HasId, values_by_id::ValuesById,
};
use serde::Deserialize;

/// Returns the current time in seconds
#[derive(Debug, Deserialize)]
pub(crate) struct TimeNode {
    id: usize,
    #[serde(skip)]
    value: f32,
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
