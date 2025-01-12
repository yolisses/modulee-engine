use crate::{node_trait::NodeTrait, node_values::NodeValues, sample_rate::SAMPLE_RATE};

#[derive(Debug)]
pub(crate) struct InputIds {}

/// Returns the current time in seconds
#[derive(Debug)]
pub(crate) struct TimeNode {
    value: f32,
    pub(crate) id: usize,
    pub(crate) input_ids: InputIds,
}

impl TimeNode {
    pub(crate) fn new(id: usize) -> Self {
        Self {
            id,
            value: 0.,
            input_ids: InputIds {},
        }
    }
}

impl NodeTrait for TimeNode {
    fn process(&mut self, _node_values: &NodeValues) -> f32 {
        self.value += 1. / SAMPLE_RATE;
        self.value
    }

    fn get_id(&self) -> usize {
        self.id
    }
}
