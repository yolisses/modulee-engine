use crate::control_update_data::ControlUpdateData;
use crate::envelope::curve::Curve;
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
    curve: Option<Curve>,
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
            let from = self
                .curve
                .as_ref()
                .map_or(self.extras.value, |c| c.get_value());
            self.curve = Some(Curve::new(from, new_value, SLEW_DURATION, SAMPLE_RATE));
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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_node(value: f32) -> ControlNode {
        ControlNode {
            id: 1,
            extras: Extras { value },
            curve: None,
        }
    }

    #[test]
    fn test_update_sets_curve_when_value_changes() {
        let mut node = make_node(0.0);
        let new_node = make_node(1.0);

        node.update(&new_node);

        assert!(node.curve.is_some());
        assert_eq!(node.extras.value, 1.0);
    }

    #[test]
    fn test_update_does_not_set_curve_when_value_unchanged() {
        let mut node = make_node(0.5);
        let new_node = make_node(0.5);

        node.update(&new_node);

        assert!(node.curve.is_none());
        assert_eq!(node.extras.value, 0.5);
    }

    #[test]
    fn test_process_returns_extras_value_when_no_curve() {
        let mut node = make_node(0.8);

        let result = node.process(&[]);

        assert_eq!(result, 0.8);
    }

    #[test]
    fn test_process_returns_curve_value_when_curve_active() {
        let mut node = make_node(0.0);
        node.curve = Some(Curve::new(0.0, 1.0, SLEW_DURATION, SAMPLE_RATE));

        let result = node.process(&[]);

        // Should not be 1.0 yet, as curve is not finished
        assert!(result > 0.0 && result < 1.0);
    }

    #[test]
    fn test_process_returns_extras_value_when_curve_finished() {
        let mut node = make_node(0.0);
        node.curve = Some(Curve::new(0.0, 1.0, 0.0, SAMPLE_RATE)); // instant curve

        // process once to finish the curve
        node.process(&[]);
        let result = node.process(&[]);

        assert_eq!(result, node.extras.value);
    }
}
