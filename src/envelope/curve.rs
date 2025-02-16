use super::get_limited_value::get_limited_value;

#[derive(Debug, Clone)]
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

    pub(crate) fn update_duration(&mut self, duration: f32) {
        let current_value_ratio = (self.current_value - self.from) / (self.to - self.from);
        let remaining_duration = duration * (1. - current_value_ratio);
        let difference = self.to - self.current_value;
        let step = difference / (remaining_duration * self.sample_rate);
        self.step = step;
    }
}

#[cfg(test)]
mod tests {
    use super::Curve;

    fn get_test_values(curve: &mut Curve, iterations: usize) -> Vec<f32> {
        let mut values = vec![];
        for _ in 0..iterations {
            curve.process();
            values.push(curve.get_value());
        }
        values
    }

    #[test]
    fn test_ascending_curve() {
        let mut curve = Curve::new(2., 5., 3., 1.);
        let values = get_test_values(&mut curve, 5);
        assert_eq!(values, vec![3., 4., 5., 5., 5.]);

        let mut curve = Curve::new(2., 5., 3., 2.);
        let values = get_test_values(&mut curve, 8);
        assert_eq!(values, vec![2.5, 3., 3.5, 4., 4.5, 5., 5., 5.]);
    }

    #[test]
    fn test_descending_curve() {
        let mut curve = Curve::new(5., 2., 3., 1.);
        let values = get_test_values(&mut curve, 5);
        assert_eq!(values, vec![4., 3., 2., 2., 2.]);

        let mut curve = Curve::new(5., 2., 3., 2.);
        let values = get_test_values(&mut curve, 8);
        assert_eq!(values, vec![4.5, 4., 3.5, 3., 2.5, 2., 2., 2.]);
    }

    #[test]
    fn test_constant_curve() {
        let mut curve = Curve::new(4., 4., 3., 1.);
        let values = get_test_values(&mut curve, 5);
        assert_eq!(values, vec![4., 4., 4., 4., 4.]);

        let mut curve = Curve::new(4., 4., 3., 2.);
        let values = get_test_values(&mut curve, 8);
        assert_eq!(values, vec![4., 4., 4., 4., 4., 4., 4., 4.]);
    }

    #[test]
    fn test_update_duration_with_ascending_curve() {
        let mut curve = Curve::new(10., 17., 7., 1.);
        let values = get_test_values(&mut curve, 3);
        assert_eq!(values, vec![11., 12., 13.]);

        // Multiply the duration by 2, divides the speed by 2
        curve.update_duration(14.);
        let values = get_test_values(&mut curve, 10);
        assert_eq!(
            values,
            vec![13.5, 14., 14.5, 15., 15.5, 16., 16.5, 17., 17., 17.]
        );
    }

    #[test]
    fn test_update_duration_with_ascending_curve_using_sample_rate() {
        let mut curve = Curve::new(10., 17., 7., 2.);
        let values = get_test_values(&mut curve, 6);
        assert_eq!(values, vec![10.5, 11., 11.5, 12., 12.5, 13.]);

        // Multiply the duration by 2, divides the speed by 2
        curve.update_duration(14.);
        let values = get_test_values(&mut curve, 20);
        assert_eq!(
            values,
            vec![
                13.25, 13.5, 13.75, 14., 14.25, 14.5, 14.75, 15., 15.25, 15.5, 15.75, 16., 16.25,
                16.5, 16.75, 17., 17., 17., 17., 17.
            ]
        );
    }

    #[test]
    fn test_update_duration_with_descending_curve() {
        let mut curve = Curve::new(17., 10., 7., 1.);
        let values = get_test_values(&mut curve, 3);
        assert_eq!(values, vec![16., 15., 14.]);

        // Multiply the duration by 2, divides the speed by 2
        curve.update_duration(14.);
        let values = get_test_values(&mut curve, 10);
        assert_eq!(
            values,
            vec![13.5, 13., 12.5, 12., 11.5, 11., 10.5, 10., 10., 10.]
        );
    }

    #[test]
    fn test_update_duration_with_descending_curve_using_sample_rate() {
        let mut curve = Curve::new(17., 10., 7., 2.);
        let values = get_test_values(&mut curve, 6);
        assert_eq!(values, vec![16.5, 16., 15.5, 15., 14.5, 14.]);

        // Multiply the duration by 2, divides the speed by 2
        curve.update_duration(14.);
        let values = get_test_values(&mut curve, 20);
        assert_eq!(
            values,
            vec![
                13.75, 13.5, 13.25, 13., 12.75, 12.5, 12.25, 12., 11.75, 11.5, 11.25, 11., 10.75,
                10.5, 10.25, 10., 10., 10., 10., 10.
            ]
        );
    }
}
