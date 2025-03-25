use crate::{
    declare_empty_update, declare_get_id, declare_get_input_ids, node_trait::NodeTrait,
    values_by_id::ValuesById,
};
use rand::{rngs::ThreadRng, Rng};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct RandomNode {
    id: usize,
    #[serde(skip)]
    generator: ThreadRng,
}

declare_get_id! {RandomNode}
declare_empty_update! {RandomNode}
declare_get_input_ids! {RandomNode,}

impl NodeTrait for RandomNode {
    fn process(&mut self, _node_values: &ValuesById) -> f32 {
        self.generator.random()
    }
}
