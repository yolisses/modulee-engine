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
    fn process(&mut self, _node_values: &ValuesById) -> f32 {
        self.value += 1. / SAMPLE_RATE;
        self.value
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
