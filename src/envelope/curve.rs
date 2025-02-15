use super::get_limited_value::get_limited_value;

pub(crate) struct Curve {
    to: f32,
    from: f32,
    step: f32,
    duration: f32,
    sample_rate: f32,
    current_value: f32,
}

impl Curve {
    pub(crate) fn new(from: f32, to: f32, duration: f32, sample_rate: f32) -> Self {
        let difference = to - from;
        let step = difference / (duration * sample_rate);
        Self {
            to,
            from,
            step,
            duration,
            sample_rate,
            current_value: from,
        }
    }

    pub(crate) fn get_value(&self) -> f32 {
        self.current_value
    }

    pub(crate) fn process(&mut self) {
        self.current_value += self.step;
        self.current_value = get_limited_value(self.current_value, self.from, self.to);
    }

    pub(crate) fn get_is_finished(&self) -> bool {
        self.current_value == self.to
    }
}

#[cfg(test)]
mod tests {
    use super::Curve;

    fn get_test_values(mut curve: Curve, iterations: usize) -> Vec<f32> {
        let mut values = vec![];
        for _ in 0..iterations {
            curve.process();
            values.push(curve.get_value());
        }
        values
    }

    #[test]
    fn test_ascending_curve() {
        let curve = Curve::new(2., 5., 3., 1.);
        let values = get_test_values(curve, 5);
        assert_eq!(values, vec![3., 4., 5., 5., 5.]);

        let curve = Curve::new(2., 5., 3., 2.);
        let values = get_test_values(curve, 8);
        assert_eq!(values, vec![2.5, 3., 3.5, 4., 4.5, 5., 5., 5.]);
    }

    #[test]
    fn test_descending_curve() {
        let curve = Curve::new(5., 2., 3., 1.);
        let values = get_test_values(curve, 5);
        assert_eq!(values, vec![4., 3., 2., 2., 2.]);

        let curve = Curve::new(5., 2., 3., 2.);
        let values = get_test_values(curve, 8);
        assert_eq!(values, vec![4.5, 4., 3.5, 3., 2.5, 2., 2., 2.]);
    }

    #[test]
    fn test_constant_curve() {
        let curve = Curve::new(4., 4., 3., 1.);
        let values = get_test_values(curve, 5);
        assert_eq!(values, vec![4., 4., 4., 4., 4.]);

        let curve = Curve::new(4., 4., 3., 2.);
        let values = get_test_values(curve, 8);
        assert_eq!(values, vec![4., 4., 4., 4., 4., 4., 4., 4.]);
    }
}
