use crate::{
    declare_empty_get_input_ids, declare_empty_update, declare_get_id, node_trait::NodeTrait,
    set_sample_rate_trait::SetSampleRateTrait,
};
use serde::Deserialize;

/// Returns the current time in seconds
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct TimeNode {
    id: usize,
    #[serde(skip)]
    value: f32,
    #[serde(skip)]
    sample_rate: f32,
}

declare_get_id! {TimeNode}
declare_empty_update! {TimeNode}
declare_empty_get_input_ids! {TimeNode}

// TODO create a macro to remove this code duplication
impl SetSampleRateTrait for TimeNode {
    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
}

impl NodeTrait for TimeNode {
    fn process(&mut self, _node_values: &[f32], _external_node_values: &[f32]) -> f32 {
        self.value += 1. / self.sample_rate;
        self.value
    }
}
