use crate::{
    node_trait::NodeTrait, sample_rate::SAMPLE_RATE, sort::has_id::HasId, values_by_id::ValuesById,
};
use biquad::{Biquad, Coefficients, DirectForm1, ToHertz, Type};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct InputIds {
    input: usize,
    frequency: usize,
    resonance: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct HighPassNode {
    id: usize,
    #[serde(default = "get_default_sample_rate")]
    sample_rate: f32,
    input_ids: InputIds,
    #[serde(skip)]
    filter: Option<DirectForm1<f32>>,
}

fn get_default_sample_rate() -> f32 {
    SAMPLE_RATE
}

impl NodeTrait for HighPassNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        let input = node_values[&self.input_ids.input];
        let mut frequency = node_values[&self.input_ids.frequency];
        let mut resonance = node_values[&self.input_ids.resonance];

        if frequency <= 0. {
            frequency = f32::EPSILON;
        }

        if resonance <= 0. {
            resonance = f32::EPSILON;
        }

        let coefficients = Coefficients::<f32>::from_params(
            Type::HighPass,
            self.sample_rate.hz(),
            frequency.hz(),
            resonance,
        )
        .unwrap();

        // This code uses a reference because make sense and prevents the filter
        // from being copied
        if let Some(filter) = &mut self.filter {
            filter.update_coefficients(coefficients);
            filter.run(input)
        } else {
            let mut filter = DirectForm1::new(coefficients);
            let output = filter.run(input);
            self.filter = Some(filter);
            return output;
        }
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![
            self.input_ids.input,
            self.input_ids.frequency,
            self.input_ids.resonance,
        ]
    }

    fn update(&mut self, new_node: &Self) {
        self.input_ids = new_node.input_ids.clone();
    }
}

impl HasId for HighPassNode {
    fn get_id(&self) -> usize {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        node_trait::NodeTrait, tests::relative_eq_array::relative_eq_array,
        values_by_id::ValuesById,
    };

    use super::{HighPassNode, InputIds};

    #[test]
    fn test_update_inputs() {
        let mut node = HighPassNode {
            id: 4,
            filter: None,
            sample_rate: 8.,
            input_ids: InputIds {
                input: 1,
                frequency: 2,
                resonance: 3,
            },
        };

        let mut node_values = ValuesById::default();
        node_values.insert(2, 2.);
        node_values.insert(3, 0.5);

        let mut outputs = vec![];
        for i in 0..10 {
            let input = i as f32 % 2.;
            node_values.insert(1, input);
            let output = node.process(&node_values);
            outputs.push(output);
        }
        relative_eq_array(
            outputs,
            vec![
                0.0, 0.69570494, 0.0, 0.69570494, 0.0, 0.69570494, 0.0, 0.69570494, 0.0, 0.69570494,
            ],
        );

        node_values.insert(2, 3.);
        let mut outputs = vec![];
        for i in 0..10 {
            let input = i as f32 % 2.;
            node_values.insert(1, input);
            let output = node.process(&node_values);
            outputs.push(output);
        }
        relative_eq_array(
            outputs,
            vec![
                0.0, 0.58868104, 0.0, 0.58868104, 0.0, 0.58868104, 0.0, 0.58868104, 0.0, 0.58868104,
            ],
        );
    }
}
