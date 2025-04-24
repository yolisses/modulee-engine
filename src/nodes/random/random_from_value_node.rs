use crate::{
    declare_get_id, declare_get_input_ids_and_its_getter, declare_update,
    get_u64_seed_from_f32::get_u64_seed_from_f32, node_trait::NodeTrait,
};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
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
    fn process(&mut self, node_values: &Vec<f32>) -> f32 {
        let value = node_values[self.input_ids.value];

        let seed = get_u64_seed_from_f32(value);

        let mut generator = Xoshiro256PlusPlus::seed_from_u64(seed);
        generator.random()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        node_trait::NodeTrait,
        nodes::random::random_from_value_node::{InputIds, RandomFromValueNode},
    };
    use assert_approx_eq::assert_approx_eq;
    use nohash_hasher::IntMap;

    #[test]
    fn test_random_from_value_node() {
        let mut node = RandomFromValueNode {
            id: 1,
            input_ids: InputIds { value: 2 },
        };

        let mut node_values = IntMap::default();

        node_values.insert(2, 0.);
        assert_approx_eq!(node.process(&node_values), 0.32457525);

        node_values.insert(2, 1.);
        assert_approx_eq!(node.process(&node_values), 0.3364141);

        node_values.insert(2, 2.);
        assert_approx_eq!(node.process(&node_values), 0.9303049);

        node_values.insert(2, 3.);
        assert_approx_eq!(node.process(&node_values), 0.8127602);

        node_values.insert(2, 4.);
        assert_approx_eq!(node.process(&node_values), 0.6547286);
    }
}
