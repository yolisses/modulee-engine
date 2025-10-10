use crate::node_trait::NodeTrait;
use crate::{declare_get_id, declare_get_input_ids_and_its_getter, declare_update};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct EqualsNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {EqualsNode}
declare_update! {EqualsNode}
declare_get_input_ids_and_its_getter! {EqualsNode, input1, input2}

impl NodeTrait for EqualsNode {
    fn process(&mut self, node_values: &[f32], _external_node_values: &[f32]) -> f32 {
        let input1 = node_values[self.input_ids.input1];
        let input2 = node_values[self.input_ids.input2];
        if input1 == input2 {
            1.
        } else {
            0.
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        node_trait::NodeTrait,
        nodes::math::equals_node::{EqualsNode, InputIds},
    };

    #[test]
    fn test_equals_node() {
        let mut equals_node = EqualsNode {
            id: 1,
            input_ids: InputIds {
                input1: 0,
                input2: 1,
            },
        };

        let node_values = [10., 20.];
        assert_eq!(equals_node.process(&node_values), 0.);

        let node_values = [20., 10.];
        assert_eq!(equals_node.process(&node_values), 0.);

        let node_values = [10., 10.];
        assert_eq!(equals_node.process(&node_values), 1.);
    }
}
