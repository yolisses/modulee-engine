use crate::{
    declare_empty_get_input_ids, declare_get_id, envelope::curve::Curve, has_update::HasUpdate,
    node_trait::NodeTrait, sample_rate::SAMPLE_RATE,
};
use serde::Deserialize;

const SLEW_RATE: f32 = 0.05; // 50 ms default

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Extras {
    value: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct ControlNode {
    id: usize,
    extras: Extras,
    #[serde(skip)]
    curve: Option<Curve>,
}

declare_get_id! {ControlNode}
declare_empty_get_input_ids! {ControlNode}

impl HasUpdate for ControlNode {
    fn update(&mut self, new_node: &Self) {
        if self.extras.value != new_node.extras.value {
            let from = self
                .curve
                .as_ref()
                .map_or(self.extras.value, |c| c.get_value());
            let to = new_node.extras.value;
            self.curve = Some(Curve::new(from, to, SLEW_RATE, SAMPLE_RATE));
        }
        self.extras = new_node.extras.clone();
    }
}

impl NodeTrait for ControlNode {
    fn process(&mut self, _node_values: &[f32]) -> f32 {
        if let Some(curve) = &mut self.curve {
            if curve.get_is_finished() {
                self.extras.value
            } else {
                curve.process();
                curve.get_value()
            }
        } else {
            self.extras.value
        }
    }
}
