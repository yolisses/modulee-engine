use crate::{node_trait::NodeTrait, node_values::NodeValues};

#[derive(Debug)]
pub(crate) struct InputIds {
    pub(crate) time: usize,
    pub(crate) frequency: usize,
}

/// Returns the phase value between 0 and 1
/// given a time and a frequency
#[derive(Debug)]
pub(crate) struct PhaseNode {
    pub(crate) id: usize,
    pub(crate) input_ids: InputIds,
}

impl PhaseNode {
    pub(crate) fn new(id: usize, time: usize, frequency: usize) -> Self {
        Self {
            id,
            input_ids: InputIds { time, frequency },
        }
    }
}

impl NodeTrait for PhaseNode {
    fn process(&mut self, node_values: &NodeValues) -> f32 {
        let time = node_values.get(&self.input_ids.time).unwrap();
        let frequency = node_values.get(&self.input_ids.frequency).unwrap();

        let product = time * frequency;
        product - product.floor()
    }

    fn get_id(&self) -> usize {
        self.id
    }
}
