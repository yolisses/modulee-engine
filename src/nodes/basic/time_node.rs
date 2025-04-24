use crate::{
    declare_empty_get_input_ids, declare_empty_update, declare_get_id, node_trait::NodeTrait,
    sample_rate::SAMPLE_RATE,
};
use serde::Deserialize;

/// Returns the current time in seconds
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct TimeNode {
    id: usize,
    #[serde(skip)]
    value: f32,
}

declare_get_id! {TimeNode}
declare_empty_update! {TimeNode}
declare_empty_get_input_ids! {TimeNode}

impl NodeTrait for TimeNode {
    fn process(&mut self, _node_values: &Vec<f32>) -> f32 {
        self.value += 1. / SAMPLE_RATE;
        self.value
    }
}
