use crate::control_update_data::ControlUpdateData;
use crate::envelope::slew::Slew;
use crate::set_sample_rate_trait::SetSampleRateTrait;
use crate::{
    declare_empty_get_input_ids, declare_get_id, has_update::HasUpdate, node_trait::NodeTrait,
};
use serde::Deserialize;

const SLEW_DURATION: f32 = 0.05; // 50 ms default

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Extras {
    value: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct ControlNode {
    id: usize,
    extras: Extras,
    #[serde(skip)]
    slew: Option<Slew>,
    #[serde(skip)]
    sample_rate: f32,
}

declare_get_id! {ControlNode}
declare_empty_get_input_ids! {ControlNode}

// TODO create a macro to remove this code duplication
impl SetSampleRateTrait for ControlNode {
    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
}

impl ControlNode {
    pub(crate) fn update_control(&mut self, control_update_data: &ControlUpdateData) {
        if self.id == control_update_data.id {
            self.update_value(control_update_data.value);
        }
    }

    fn update_value(&mut self, new_value: f32) {
        if self.extras.value != new_value {
            let start = if let Some(slew) = &self.slew {
                slew.get_value()
            } else {
                self.extras.value
            };
            self.slew = Some(Slew::new(start, new_value, SLEW_DURATION, self.sample_rate));
        }
    }
}

impl HasUpdate for ControlNode {
    fn update(&mut self, new_node: &Self) {
        self.update_value(new_node.extras.value);
        self.extras = new_node.extras.clone();
    }
}

impl NodeTrait for ControlNode {
    fn process(&mut self, _node_values: &[f32], _external_node_values: &[f32]) -> f32 {
        if let Some(slew) = &mut self.slew {
            if !slew.get_is_finished() {
                slew.process();
            }
            slew.get_value()
        } else {
            self.extras.value
        }
    }
}
