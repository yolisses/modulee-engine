use super::get_bool_value::get_bool_value;
use crate::node_trait::NodeTrait;
use crate::{declare_get_id, declare_get_input_ids_and_its_getter, declare_update};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct NotNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {NotNode}
declare_update! {NotNode}
declare_get_input_ids_and_its_getter! {NotNode, input}

impl NodeTrait for NotNode {
    fn process(&mut self, node_values: &[f32]) -> f32 {
        let input = node_values[self.input_ids.input];
        if get_bool_value(input) {
            0.
        } else {
            1.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_node_truth_table() {
        let mut node = NotNode {
            id: 1,
            input_ids: InputIds { input: 0 },
        };

        let test_cases = vec![
            //
            (0., 1.),
            (1., 0.),
        ];

        for (input, expected) in test_cases {
            let node_values = vec![input];
            let result = node.process(&node_values, &Vec::default());
            assert_eq!(result, expected, "Failed for inputs: ({input})");
        }
    }
}
