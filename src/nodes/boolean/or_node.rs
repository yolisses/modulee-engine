use super::get_bool_value::get_bool_value;
use crate::node_trait::NodeTrait;
use crate::{declare_get_id, declare_get_input_ids_and_its_getter, declare_update};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct OrNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {OrNode}
declare_update! {OrNode}
declare_get_input_ids_and_its_getter! {OrNode, input1, input2}

impl NodeTrait for OrNode {
    fn process(&mut self, node_values: &[f32], _external_node_values: &[f32]) -> f32 {
        let input1 = node_values[self.input_ids.input1];
        let input2 = node_values[self.input_ids.input2];
        if get_bool_value(input1) || get_bool_value(input2) {
            1.
        } else {
            0.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_node_truth_table() {
        let mut node = OrNode {
            id: 1,
            input_ids: InputIds {
                input1: 0,
                input2: 1,
            },
        };

        let test_cases = vec![
            //
            (0., 0., 0.),
            (0., 1., 1.),
            (1., 0., 1.),
            (1., 1., 1.),
        ];

        for (input1, input2, expected) in test_cases {
            let node_values = vec![input1, input2];
            let result = node.process(&node_values, &Vec::default());
            assert_eq!(result, expected, "Failed for inputs: ({input1}, {input2})");
        }
    }
}
