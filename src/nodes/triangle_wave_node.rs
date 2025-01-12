use crate::{node_trait::NodeTrait, node_values::NodeValues};

#[derive(Debug)]
pub(crate) struct InputIds {
    pub(crate) phase: usize,
}

#[derive(Debug)]
pub(crate) struct TriangleWaveNode {
    pub(crate) id: usize,
    pub(crate) input_ids: InputIds,
}

impl TriangleWaveNode {
    pub(crate) fn new(id: usize, phase: usize) -> Self {
        Self {
            id,
            input_ids: InputIds { phase },
        }
    }
}

impl NodeTrait for TriangleWaveNode {
    fn process(&mut self, node_values: &NodeValues) -> f32 {
        let phase = node_values.get(&self.input_ids.phase).unwrap();
        let t = phase % 1.0;
        if t < 0.5 {
            4.0 * t - 1.0
        } else {
            3.0 - 4.0 * t
        }
    }

    fn get_id(&self) -> usize {
        self.id
    }
}
