use crate::{
    declare_get_id, declare_get_input_ids_and_its_getter, declare_update, node_trait::NodeTrait,
    sample_rate::SAMPLE_RATE,
};
use serde::Deserialize;

/// Returns the phase value between 0 and 1 given a time and a frequency
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct PhaseNode {
    id: usize,
    input_ids: InputIds,
    #[serde(skip)]
    phase: f32,
    #[serde(skip)]
    step: f32,
    #[serde(skip)]
    last_frequency: f32,
    #[serde(default = "get_default_sample_rate")]
    sample_rate: f32,
}

declare_get_id! {PhaseNode}
declare_update! {PhaseNode}
declare_get_input_ids_and_its_getter! {PhaseNode, frequency}

fn get_default_sample_rate() -> f32 {
    SAMPLE_RATE
}

impl NodeTrait for PhaseNode {
    fn process(&mut self, node_values: &Vec<f32>) -> f32 {
        let frequency = node_values[self.input_ids.frequency];

        if frequency != self.last_frequency {
            self.last_frequency = frequency;
            self.step = frequency / self.sample_rate;
        }

        self.phase += self.step;
        // Equals to `%= 1` but more precise
        self.phase -= self.phase.floor();
        self.phase
    }
}

#[cfg(test)]
mod tests {
    use super::PhaseNode;
    use crate::node_trait::NodeTrait;
    use crate::tests::relative_eq_array::relative_eq_array;
    use nohash_hasher::IntMap;

    fn get_test_values(
        phase_node: &mut PhaseNode,
        node_values: &Vec<f32>,
        iterations: usize,
    ) -> Vec<f32> {
        let mut values = vec![];
        for _ in 0..iterations {
            phase_node.process(node_values);
            values.push(phase_node.phase);
        }
        values
    }

    #[test]
    fn test_phase_node() {
        let frequency_id = 1;
        let mut phase_node = PhaseNode {
            id: 0,
            step: 0.,
            phase: 0.,
            sample_rate: 50.,
            last_frequency: 0.,
            input_ids: super::InputIds {
                frequency: frequency_id,
            },
        };

        let frequency = 10.;
        let mut node_values = IntMap::default();
        node_values.insert(frequency_id, frequency);

        relative_eq_array(
            get_test_values(&mut phase_node, &node_values, 10),
            vec![0.2, 0.4, 0.6, 0.8, 0.0, 0.2, 0.4, 0.6, 0.8, 0.0],
        );
    }
}
