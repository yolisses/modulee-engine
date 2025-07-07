use crate::math::get_clamped_value::get_clamped_value;
use serde::Deserialize;

/// Delay line implemented with a limited size queue.
///
/// It stores a least one value, to prevent returning zero when `max_time` is
/// zero
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Delay {
    buffer: Vec<f32>,
    sample_rate: f32,
}

impl Delay {
    pub(crate) fn new(max_time: f32, sample_rate: f32) -> Self {
        let mut it = Self {
            sample_rate,
            buffer: vec![],
        };
        it.update_max_time(max_time);
        it
    }

    pub(crate) fn push_input(&mut self, input: f32) {
        if !self.buffer.is_empty() {
            self.buffer.remove(0);
        }
        self.buffer.push(input);
    }

    pub(crate) fn get_value(&self, time: f32) -> f32 {
        let mut index = (time * self.sample_rate) as usize;
        index = get_clamped_value(index, 0, self.buffer.len() - 1);
        self.buffer[index]
    }

    /// Returns true if there is some non zero value in the delay buffer
    pub(crate) fn get_is_pending(&self) -> bool {
        self.buffer.iter().any(|&x| x != 0.)
    }

    /// Updates the maximum delay time by resizing the buffer
    pub(crate) fn update_max_time(&mut self, max_time: f32) {
        let mut new_size = (max_time * self.sample_rate) as usize;
        if new_size < 1 {
            new_size = 1;
        }
        let current_size = self.buffer.len();

        if new_size != current_size {
            let mut new_buffer = vec![0.; new_size];

            // Copy existing data to the new buffer, preserving as much as possible
            if new_size > current_size {
                // If expanding, copy all existing data and pad with zeros
                new_buffer[..current_size].copy_from_slice(&self.buffer);
            } else {
                // If shrinking, copy only the most recent samples
                let start = current_size - new_size;
                new_buffer.copy_from_slice(&self.buffer[start..]);
            }

            self.buffer = new_buffer;
        }
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
    #[test]
    fn test_update_max_time_expand() {
        let mut delay = Delay::new(2., 1.);
        delay.push_input(1.);
        delay.push_input(2.);
        delay.update_max_time(4.);
        assert_array_approx_eq(&delay.buffer, &vec![1., 2., 0., 0.]);
    }

    #[test]
    fn test_update_max_time_shrink() {
        let mut delay = Delay::new(4., 1.);
        delay.push_input(1.);
        delay.push_input(2.);
        delay.push_input(3.);
        delay.push_input(4.);
        delay.update_max_time(2.);
        assert_array_approx_eq(&delay.buffer, &vec![3., 4.]);
    }

    #[test]
    fn test_update_max_time_no_change() {
        let mut delay = Delay::new(2., 1.);
        delay.push_input(1.);
        delay.push_input(2.);
        let original_buffer = delay.buffer.clone();
        delay.update_max_time(2.);
        assert_array_approx_eq(&delay.buffer, &original_buffer);
    }

    #[test]
    fn test_delay_with_max_time_zero() {
        let mut delay = Delay::new(0., 1.);

        assert_array_approx_eq(&delay.buffer, &vec![0.]);

        delay.push_input(1.);

        assert_array_approx_eq(&delay.buffer, &vec![1.]);

        assert_approx_eq!(delay.get_value(-1.), 1.);
        assert_approx_eq!(delay.get_value(0.), 1.);
        assert_approx_eq!(delay.get_value(1.), 1.);
    }
}
