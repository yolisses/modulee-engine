use crate::node_trait::NodeTrait;
use crate::{declare_get_id, declare_get_input_ids_and_its_getter, declare_update};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct AddNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {AddNode}
declare_update! {AddNode}
declare_get_input_ids_and_its_getter! {AddNode, input1, input2}

impl NodeTrait for AddNode {
    fn process(&mut self, node_values: &[f32]) -> f32 {
        let input1 = node_values[self.input_ids.input1];
        let input2 = node_values[self.input_ids.input2];
        input1 + input2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node_process() {
        let input_ids = InputIds {
            input1: 0,
            input2: 1,
        };
        let mut add_node = AddNode { id: 1, input_ids };

        let node_values = vec![2., 3.];
        let result = add_node.process(&node_values, &Vec::default());

        assert_eq!(result, 5.);
    }

    #[test]
    fn test_add_node_with_negative_values() {
        let input_ids = InputIds {
            input1: 0,
            input2: 1,
        };
        let mut add_node = AddNode { id: 2, input_ids };

        let node_values = vec![-4., -6.];
        let result = add_node.process(&node_values, &Vec::default());

        assert_eq!(result, -10.);
    }

    #[test]
    fn test_add_node_with_mixed_values() {
        let input_ids = InputIds {
            input1: 0,
            input2: 1,
        };
        let mut add_node = AddNode { id: 3, input_ids };

        let node_values = vec![7.5, -2.5];
        let result = add_node.process(&node_values, &Vec::default());

        assert_eq!(result, 5.);
    }

    #[test]
    fn test_add_node_with_zero_values() {
        let input_ids = InputIds {
            input1: 0,
            input2: 1,
        };
        let mut add_node = AddNode { id: 4, input_ids };

        let node_values = vec![0., 0.];
        let result = add_node.process(&node_values, &Vec::default());

        assert_eq!(result, 0.);
    }
}
