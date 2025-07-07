use crate::{
    declare_get_id, declare_get_input_ids_and_its_getter, delay::delay::Delay,
    has_update::HasUpdate, node_trait::NodeTrait, sample_rate::SAMPLE_RATE,
};
use serde::Deserialize;

// TODO consider adding gate, to allow delay starts different than note
// starts.
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct DelayNode {
    id: usize,
    input_ids: InputIds,
    #[serde(skip)]
    #[serde(default = "get_default_delay")]
    delay: Delay,
}

fn get_default_delay() -> Delay {
    Delay::new(0., SAMPLE_RATE)
}

declare_get_id! {DelayNode}
declare_get_input_ids_and_its_getter! {DelayNode, input, time, max_time}

impl HasUpdate for DelayNode {
    // TODO check if makes sense to clone the delay too
    fn update(&mut self, new_node: &Self) {
        self.input_ids = new_node.input_ids.clone();
    }
}

impl NodeTrait for DelayNode {
    fn process(&mut self, node_values: &[f32]) -> f32 {
        let input = node_values[self.input_ids.input];
        let time = node_values[self.input_ids.time];
        let max_time = node_values[self.input_ids.max_time];
        // self.delay.update_parameters(input, time, max_time);

        // self.delay.process();
        // self.delay.get_value()
        // DEBUG
        0.
    }

    fn get_is_pending(&self) -> bool {
        // self.delay.get_is_pending()
        // DEBUG
        false
    }
}
