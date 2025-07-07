use crate::math::get_clamped_value::get_clamped_value;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Delay {
    buffer: Vec<f32>,
    sample_rate: f32,
}

impl Delay {
    pub(crate) fn new(max_time: f32, sample_rate: f32) -> Self {
        Self {
            sample_rate,
            buffer: vec![0.; (max_time * sample_rate) as usize],
        }
    }

    pub(crate) fn push_input(&mut self, input: f32) {
        self.buffer.remove(0);
        self.buffer.push(input);
    }

    pub(crate) fn get_value(&self, time: f32) -> f32 {
        let mut index = (time * self.sample_rate) as usize;
        index = get_clamped_value(index, 0, self.buffer.len() - 1);
        self.buffer[index]
    }
}

#[cfg(test)]
mod tests {
    use super::Delay;
    use crate::tests::assert_array_approx_eq::assert_array_approx_eq;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_delay() {
        let mut delay = Delay::new(4., 1.);
        delay.push_input(1.);
        delay.push_input(2.);
        delay.push_input(3.);
        delay.push_input(4.);

        assert_array_approx_eq(&delay.buffer, &vec![1., 2., 3., 4.]);
        assert_approx_eq!(delay.get_value(-1.), 1.);
        assert_approx_eq!(delay.get_value(0.), 1.);
        assert_approx_eq!(delay.get_value(1.), 2.);
        assert_approx_eq!(delay.get_value(2.), 3.);
        assert_approx_eq!(delay.get_value(3.), 4.);
        assert_approx_eq!(delay.get_value(4.), 4.);
    }
}
