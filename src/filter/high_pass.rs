use crate::sample_rate::SAMPLE_RATE;
use biquad::{Biquad, Coefficients, DirectForm1, ToHertz, Type};

#[derive(Debug, Clone)]
pub struct HighPass {
    sample_rate: f32,
    last_frequency: f32,
    last_resonance: f32,
    nyquist_frequency: f32,
    filter: Option<DirectForm1<f32>>,
}

impl Default for HighPass {
    fn default() -> Self {
        Self {
            last_frequency: -1.,
            last_resonance: -1.,
            sample_rate: SAMPLE_RATE,
            filter: Default::default(),
            nyquist_frequency: SAMPLE_RATE / 2.,
        }
    }
}

impl HighPass {
    pub fn process(&mut self, input: f32, mut frequency: f32, mut resonance: f32) -> f32 {
        if frequency != self.last_frequency || resonance != self.last_resonance {
            if frequency <= 0. {
                frequency = f32::EPSILON;
            }

            if frequency > self.nyquist_frequency {
                frequency = self.nyquist_frequency;
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

            // This code uses a reference because make sense and prevents the
            // filter from being copied
            if let Some(filter) = &mut self.filter {
                filter.update_coefficients(coefficients);
            } else {
                let filter = DirectForm1::new(coefficients);
                self.filter = Some(filter);
            }
        }

        if let Some(filter) = &mut self.filter {
            filter.run(input)
        } else {
            panic!("Implementation error");
        }
    }
}
