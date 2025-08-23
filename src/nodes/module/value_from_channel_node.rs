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
    #[serde(skip)]
    value: f32,
}

declare_get_id! {ValueFromChannelNode}
declare_update! {ValueFromChannelNode}
declare_get_input_ids_and_its_getter! {ValueFromChannelNode, input}

impl ValueFromChannelNode {
    pub(crate) fn get_channel(&mut self) -> u8 {
        self.extras.channel
    }

    pub(crate) fn get_input_id(&mut self) -> usize {
        self.input_ids.input
    }

    pub(crate) fn set_value(&mut self, value: f32) {
        self.value = value;
    }
}

impl NodeTrait for ValueFromChannelNode {
    fn process(&mut self, _node_values: &[f32]) -> f32 {
        self.value
    }
}
