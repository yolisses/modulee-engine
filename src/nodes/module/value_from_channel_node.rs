use crate::{
    declare_get_id, declare_get_input_ids_and_its_getter, declare_update, node_trait::NodeTrait,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Extras {
    channel: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct ValueFromChannelNode {
    extras: Extras,
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {ValueFromChannelNode}
declare_update! {ValueFromChannelNode}
declare_get_input_ids_and_its_getter! {ValueFromChannelNode, input}

impl NodeTrait for ValueFromChannelNode {
    fn process(&mut self, node_values: &[f32], _external_node_values: &[f32]) -> f32 {
        let mut index = self.input_ids.input;
        index += self.extras.channel as usize;
        node_values[index]
    }
}
