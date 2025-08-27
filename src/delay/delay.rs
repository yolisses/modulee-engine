use crate::{
    math::get_clamped_value::get_clamped_value, set_sample_rate_trait::SetSampleRateTrait,
};
use serde::Deserialize;
use std::collections::VecDeque;

/// Delay line implemented with a limited size queue.
///
/// It stores at least one value, to prevent returning zero when `max_time` is
/// zero
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Delay {
    sample_rate: f32,
    buffer: VecDeque<f32>,
}

impl SetSampleRateTrait for Delay {
    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
}

impl Delay {
    pub(crate) fn new(max_time: f32, sample_rate: f32) -> Self {
        let mut it = Self {
            sample_rate,
            buffer: VecDeque::new(),
        };
        it.update_max_time(max_time);
        it
    }

    pub(crate) fn push_input(&mut self, input: f32) {
        if !self.buffer.is_empty() {
            self.buffer.pop_front();
        }
        self.buffer.push_back(input);
    }

    pub(crate) fn get_value(&self, time: f32) -> f32 {
        let mut index = (time * self.sample_rate) as usize;
        index = get_clamped_value(index, 0, self.buffer.len().saturating_sub(1));
        // Index from the back: 0 = newest, 1 = previous, etc.
        self.buffer[self.buffer.len() - 1 - index]
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
            if new_size > current_size {
                // If expanding, pad with zeros at the back
                for _ in 0..(new_size - current_size) {
                    self.buffer.push_back(0.);
                }
            } else {
                // If shrinking, keep only the most recent samples
                for _ in 0..(current_size - new_size) {
                    self.buffer.pop_front();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Delay;
    use crate::tests::assert_array_approx_eq::assert_array_approx_eq;
    use assert_approx_eq::assert_approx_eq;

    fn deque_to_vec(d: &std::collections::VecDeque<f32>) -> Vec<f32> {
        d.iter().copied().collect()
    }

    #[test]
    fn test_delay() {
        let mut delay = Delay::new(4., 1.);
        delay.push_input(5.);
        delay.push_input(6.);
        delay.push_input(7.);
        delay.push_input(8.);

        assert_array_approx_eq(&deque_to_vec(&delay.buffer), &vec![5., 6., 7., 8.]);
        assert_approx_eq!(delay.get_value(-1.), 8.);
        assert_approx_eq!(delay.get_value(0.), 8.);
        assert_approx_eq!(delay.get_value(1.), 7.);
        assert_approx_eq!(delay.get_value(2.), 6.);
        assert_approx_eq!(delay.get_value(3.), 5.);
        assert_approx_eq!(delay.get_value(4.), 5.);
    }
    #[test]
    fn test_update_max_time_expand() {
        let mut delay = Delay::new(2., 1.);
        delay.push_input(1.);
        delay.push_input(2.);
        delay.update_max_time(4.);
        assert_array_approx_eq(&deque_to_vec(&delay.buffer), &vec![1., 2., 0., 0.]);
    }

    #[test]
    fn test_update_max_time_shrink() {
        let mut delay = Delay::new(4., 1.);
        delay.push_input(1.);
        delay.push_input(2.);
        delay.push_input(3.);
        delay.push_input(4.);
        delay.update_max_time(2.);
        assert_array_approx_eq(&deque_to_vec(&delay.buffer), &vec![3., 4.]);
    }

    #[test]
    fn test_update_max_time_no_change() {
        let mut delay = Delay::new(2., 1.);
        delay.push_input(1.);
        delay.push_input(2.);
        let original_buffer = delay.buffer.clone();
        delay.update_max_time(2.);
        assert_array_approx_eq(
            &deque_to_vec(&delay.buffer),
            &deque_to_vec(&original_buffer),
        );
    }

    #[test]
    fn test_delay_with_max_time_zero() {
        let mut delay = Delay::new(0., 1.);

        assert_array_approx_eq(&deque_to_vec(&delay.buffer), &vec![0.]);

        delay.push_input(1.);

        assert_array_approx_eq(&deque_to_vec(&delay.buffer), &vec![1.]);

        assert_approx_eq!(delay.get_value(-1.), 1.);
        assert_approx_eq!(delay.get_value(0.), 1.);
        assert_approx_eq!(delay.get_value(1.), 1.);
    }
}
