use crate::{
    declare_get_id, declare_get_input_ids_and_its_getter, declare_update, node_trait::NodeTrait,
    nodes::boolean::get_bool_value::get_bool_value,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct HoldNode {
    id: usize,
    #[serde(skip)]
    value: f32,
    input_ids: InputIds,
}

declare_get_id! {HoldNode}
declare_update! {HoldNode}
declare_get_input_ids_and_its_getter! {HoldNode, input, trigger}

impl NodeTrait for HoldNode {
    fn process(&mut self, node_values: &[f32], _external_node_values: &[f32]) -> f32 {
        let input = node_values[self.input_ids.input];
        let trigger = node_values[self.input_ids.trigger];

        if get_bool_value(trigger) {
            self.value = input;
        }

        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hold_node_process_toggle_trigger() {
        let mut hold_node = HoldNode {
            id: 1,
            value: 0.,
            input_ids: InputIds {
                input: 0,
                trigger: 1,
            },
        };

        let mut node_values = vec![42., 1.];

        let result = hold_node.process(&node_values, &Vec::default());
        assert_eq!(result, 42.);
        assert_eq!(hold_node.value, 42.);

        node_values = vec![81., 0.];

        let result = hold_node.process(&node_values, &Vec::default());
        assert_eq!(result, 42.);
        assert_eq!(hold_node.value, 42.);

        node_values = vec![84., 1.];

        let result = hold_node.process(&node_values, &Vec::default());
        assert_eq!(result, 84.);
        assert_eq!(hold_node.value, 84.);
    }
}
