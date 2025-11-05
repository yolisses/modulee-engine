use crate::node_trait::NodeTrait;
use crate::{declare_get_id, declare_get_input_ids_and_its_getter, declare_update};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct CeilNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {CeilNode}
declare_update! {CeilNode}
declare_get_input_ids_and_its_getter! {CeilNode, input}

impl NodeTrait for CeilNode {
    fn process(&mut self, node_values: &[f32], _external_node_values: &[f32]) -> f32 {
        let input = unsafe { *node_values.get_unchecked(self.input_ids.input) };
        input.ceil()
    }
}
