use crate::math::get_clamped_value::get_clamped_value;
use biquad::{Biquad, Coefficients, DirectForm1, ToHertz, Type};
use core::f32;

// TODO consider replacing this by FilterWrapperWithGain
#[derive(Debug, Clone)]
pub(crate) struct FilterWrapper {
    sample_rate: f32,
    filter_type: Type<f32>,
    nyquist_frequency: f32,
    filter: Option<DirectForm1<f32>>,
}

impl FilterWrapper {
    pub(crate) fn new(filter_type: Type<f32>, sample_rate: f32) -> Self {
        Self {
            filter_type,
            filter: Default::default(),
            nyquist_frequency: sample_rate / 2.,
            sample_rate,
        }
    }

    pub fn process(&mut self, input: f32, mut frequency: f32, mut resonance: f32) -> f32 {
        frequency = get_clamped_value(frequency, f32::EPSILON, self.nyquist_frequency);
        resonance = resonance.max(f32::EPSILON);

        let coefficients = Coefficients::<f32>::from_params(
            self.filter_type,
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
            output
        }
    }
}
