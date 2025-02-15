// TODO consider throwing an error if min is greater than max
pub(crate) fn get_clamped_value(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        return min;
    }

    if value > max {
        return max;
    }

    value
}
#[cfg(test)]
mod tests {
    use crate::envelope::get_clamped_value::get_clamped_value;

    #[test]
    fn test_get_clamped_value_with_value_in_between() {
        assert_eq!(get_clamped_value(3., 2., 4.), 3.);
    }

    #[test]
    fn test_get_clamped_value_with_value_less_than_min() {
        assert_eq!(get_clamped_value(1., 2., 4.), 2.);
    }

    #[test]
    fn test_get_clamped_value_with_value_greater_than_max() {
        assert_eq!(get_clamped_value(5., 2., 4.), 4.);
    }
}
