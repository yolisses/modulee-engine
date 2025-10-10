use super::get_bool_value::get_bool_value;
use crate::node_trait::NodeTrait;
use crate::{declare_get_id, declare_get_input_ids_and_its_getter, declare_update};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct IfNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {IfNode}
declare_update! {IfNode}
declare_get_input_ids_and_its_getter! {IfNode, condition, input1, input2}

impl NodeTrait for IfNode {
    fn process(&mut self, node_values: &[f32], _external_node_values: &[f32]) -> f32 {
        let input1 = node_values[self.input_ids.input1];
        let input2 = node_values[self.input_ids.input2];
        let condition = node_values[self.input_ids.condition];
        if get_bool_value(condition) {
            input1
        } else {
            input2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_node_condition_true() {
        let mut node = IfNode {
            id: 1,
            input_ids: InputIds {
                condition: 0,
                input1: 1,
                input2: 2,
            },
        };

        let node_values = vec![1.0, 42.0, 63.0];
        let result = node.process(&node_values);
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_if_node_condition_false() {
        let mut node = IfNode {
            id: 1,
            input_ids: InputIds {
                condition: 0,
                input1: 1,
                input2: 2,
            },
        };

        let node_values = vec![0.0, 42.0, 63.0];
        let result = node.process(&node_values);
        assert_eq!(result, 63.0);
    }
}
