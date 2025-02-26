use crate::{node_trait::NodeTrait, sort::has_id::HasId, values_by_id::ValuesById};
use rand::{rngs::ThreadRng, Rng};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct RandomNode {
    id: usize,
    #[serde(skip)]
    generator: ThreadRng,
}

impl NodeTrait for RandomNode {
    fn process(&mut self, _node_values: &ValuesById) -> f32 {
        self.generator.random()
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![]
    }

    fn update(&mut self, _new_node: &Self) {}
}

impl HasId for RandomNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
