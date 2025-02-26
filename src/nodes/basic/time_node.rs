use crate::{
    node_trait::NodeTrait, sample_rate::SAMPLE_RATE, sort::has_id::HasId, values_by_id::ValuesById,
};
use serde::Deserialize;

/// Returns the current time in seconds
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct TimeNode {
    id: usize,
    #[serde(skip)]
    value: f32,
}

impl NodeTrait for TimeNode {
    fn process(&mut self, _node_values: &ValuesById) -> f32 {
        self.value += 1. / SAMPLE_RATE;
        self.value
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![]
    }

    fn update(&mut self, _new_node: &Self) {}
}

impl HasId for TimeNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
