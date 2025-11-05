use crate::{
    declare_get_id, declare_get_input_ids_and_its_getter, declare_update, node_trait::NodeTrait,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct RandomFromValueNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {RandomFromValueNode}
declare_update! {RandomFromValueNode}
declare_get_input_ids_and_its_getter! {RandomFromValueNode, value}

impl NodeTrait for RandomFromValueNode {
    fn process(&mut self, node_values: &[f32], _external_node_values: &[f32]) -> f32 {
        let value = unsafe { *node_values.get_unchecked(self.input_ids.value) };
        let bits = value.to_bits();
        let mut seed = bits as u64;
        seed = seed.wrapping_mul(0x517cc1b727220a95);
        seed ^= seed >> 30;
        seed = seed.wrapping_mul(0x517cc1b727220a95);
        ((seed as u32) as f32) / u32::MAX as f32
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        node_trait::NodeTrait,
        nodes::random::random_from_value_node::{InputIds, RandomFromValueNode},
    };
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_random_from_value_node() {
        let mut node = RandomFromValueNode {
            id: 1,
            input_ids: InputIds { value: 0 },
        };

        let mut node_values: Vec<f32> = vec![0.];

        node_values[0] = 0.;
        assert_approx_eq!(node.process(&node_values, &Vec::default()), 0.);

        node_values[0] = 1.;
        assert_approx_eq!(node.process(&node_values, &Vec::default()), 0.98976886);

        node_values[0] = 2.;
        assert_approx_eq!(node.process(&node_values, &Vec::default()), 0.4609982);

        node_values[0] = 3.;
        assert_approx_eq!(node.process(&node_values, &Vec::default()), 0.8679685);

        node_values[0] = 4.;
        assert_approx_eq!(node.process(&node_values, &Vec::default()), 0.32233316);
    }
}
