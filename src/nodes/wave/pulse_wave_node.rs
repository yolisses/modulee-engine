use crate::node_trait::NodeTrait;
use crate::{declare_get_id, declare_get_input_ids_and_its_getter, declare_update};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct PulseWaveNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {PulseWaveNode}
declare_update! {PulseWaveNode}
declare_get_input_ids_and_its_getter! {PulseWaveNode, phase, duty_cycle}

impl NodeTrait for PulseWaveNode {
    fn process(&mut self, node_values: &[f32], _external_node_values: &[f32]) -> f32 {
        let phase = node_values[self.input_ids.phase];
        let duty_cycle = node_values[self.input_ids.duty_cycle];

        if phase < duty_cycle {
            1.0
        } else {
            -1.0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        node_trait::NodeTrait,
        nodes::wave::pulse_wave_node::{InputIds, PulseWaveNode},
    };

    #[test]
    fn test_pulse_wave_node() {
        let mut node = PulseWaveNode {
            id: 1,
            input_ids: InputIds {
                phase: 0,
                duty_cycle: 1,
            },
        };

        let node_values = [0.3, 0.5]; // phase < duty_cycle
        assert_eq!(node.process(&node_values, &Vec::default()), 1.0);

        let node_values = [0.7, 0.5]; // phase > duty_cycle
        assert_eq!(node.process(&node_values, &Vec::default()), -1.0);

        let node_values = [0.5, 0.5]; // phase == duty_cycle
        assert_eq!(node.process(&node_values, &Vec::default()), -1.0);
    }
}
