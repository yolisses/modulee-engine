use crate::node_trait::NodeTrait;
use crate::{declare_get_id, declare_get_input_ids_and_its_getter, declare_update};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct ModuloNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {ModuloNode}
declare_update! {ModuloNode}
declare_get_input_ids_and_its_getter! {ModuloNode, input1, input2}

impl NodeTrait for ModuloNode {
    fn process(&mut self, node_values: &[f32]) -> f32 {
        let input1 = node_values[self.input_ids.input1];
        let input2 = node_values[self.input_ids.input2];
        let result = input1 % input2;
        if result.is_nan() {
            0.
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn test_modulo_node_process() {
        let input_ids = InputIds {
            input1: 0,
            input2: 1,
        };
        let mut node = ModuloNode { id: 1, input_ids };

        let node_values = vec![10., 3.];
        let result = node.process(&node_values);
        assert_eq!(result, 1.);
    }

    #[test]
    fn test_modulo_node_process_with_zero_divisor() {
        let input_ids = InputIds {
            input1: 0,
            input2: 1,
        };
        let mut node = ModuloNode { id: 1, input_ids };

        let node_values = vec![10., 0.];
        let result = node.process(&node_values);
        assert_eq!(result, 0.);
    }

    #[test]
    fn test_modulo_node_process_negative_numbers() {
        let input_ids = InputIds {
            input1: 0,
            input2: 1,
        };
        let mut node = ModuloNode { id: 1, input_ids };

        let node_values = vec![-10., 3.];
        let result = node.process(&node_values);
        assert_eq!(result, -1.);
    }

    #[test]
    fn test_modulo_node_process_with_floats() {
        let input_ids = InputIds {
            input1: 0,
            input2: 1,
        };
        let mut node = ModuloNode { id: 1, input_ids };

        let node_values = vec![10.5, 3.2];
        let result = node.process(&node_values);
        assert_approx_eq!(result, 0.9);
    }
}
