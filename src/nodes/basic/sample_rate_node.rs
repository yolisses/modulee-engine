use crate::{
    declare_empty_get_input_ids, declare_empty_update, declare_get_id,
    default_sample_rate::DEFAULT_SAMPLE_RATE, node_trait::NodeTrait,
    set_sample_rate_trait::SetSampleRateTrait,
};
use serde::Deserialize;

// TODO consider adding gate, to allow sample_rate starts different than note
// starts.
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct SampleRateNode {
    id: usize,
    #[serde(skip)]
    #[serde(default = "get_default_sample_rate")]
    sample_rate: f32,
}

fn get_default_sample_rate() -> f32 {
    DEFAULT_SAMPLE_RATE
}

declare_get_id! {SampleRateNode}
declare_empty_update! {SampleRateNode}
declare_empty_get_input_ids! {SampleRateNode}

impl NodeTrait for SampleRateNode {
    fn process(&mut self, _node_values: &[f32], _external_node_values: &[f32]) -> f32 {
        self.sample_rate
    }
}

impl SetSampleRateTrait for SampleRateNode {
    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
}
