use crate::control_update_data::ControlUpdateData;
use crate::envelope::slew::Slew;
use crate::{
    declare_empty_get_input_ids, declare_get_id, has_update::HasUpdate, node_trait::NodeTrait,
    sample_rate::SAMPLE_RATE,
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
}

declare_get_id! {ControlNode}
declare_empty_get_input_ids! {ControlNode}

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
            self.slew = Some(Slew::new(start, new_value, SLEW_DURATION, SAMPLE_RATE));
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
    fn process(&mut self, _node_values: &[f32]) -> f32 {
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
