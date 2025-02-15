use super::get_limited_value::get_limited_value;

struct Curve {
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
}

#[cfg(test)]
mod tests {
    use super::Curve;

    #[test]
    fn test_ascending_curve() {
        let mut curve = Curve::new(2., 5., 3., 1.);

        let mut values = vec![];
        for _ in 0..5 {
            curve.process();
            values.push(curve.get_value());
        }

        assert_eq!(values, vec![3., 4., 5., 5., 5.])
    }
}
