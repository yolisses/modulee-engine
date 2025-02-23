use crate::{
    node_trait::NodeTrait, sample_rate::SAMPLE_RATE, sort::has_id::HasId, values_by_id::ValuesById,
};
use biquad::{Biquad, Coefficients, DirectForm1, ToHertz, Type, Q_BUTTERWORTH_F32};
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
    input_ids: InputIds,
    #[serde(skip)]
    filter: Option<DirectForm1<f32>>,
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

        frequency = 1000.;
        resonance = Q_BUTTERWORTH_F32;

        let coefficients = Coefficients::<f32>::from_params(
            Type::HighPass,
            SAMPLE_RATE.hz(),
            frequency.hz(),
            resonance,
        )
        .unwrap();

        if let Some(mut filter) = self.filter {
            filter.update_coefficients(coefficients);
            filter.run(input)
        } else {
            let mut filter = DirectForm1::new(coefficients);
            let result = filter.run(input);
            self.filter = Some(filter);
            result
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
