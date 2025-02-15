use super::get_clamped_value::get_clamped_value;

/// Returns the value clamped by the lesser and greatest limits.
///
/// It's basically a `get_clamped_value` where the order of the limits doesn't
/// matter.
fn get_limited_value(value: f32, limit_1: f32, limit_2: f32) -> f32 {
    if limit_1 < limit_2 {
        return get_clamped_value(value, limit_1, limit_2);
    }

    if limit_1 > limit_2 {
        return get_clamped_value(value, limit_2, limit_1);
    }

    // The limits are equal
    return limit_1;
}

#[cfg(test)]
mod tests {
    use crate::envelope::get_limited_value::get_limited_value;

    #[test]
    fn test_get_limited_value_with_limit_1_less_than_limit_2() {
        assert_eq!(get_limited_value(1., 2., 4.), 2.);
        assert_eq!(get_limited_value(3., 2., 4.), 3.);
        assert_eq!(get_limited_value(5., 2., 4.), 4.);
    }

    #[test]
    fn test_get_limited_value_with_limit_1_greater_than_limit_2() {
        assert_eq!(get_limited_value(1., 4., 2.), 2.);
        assert_eq!(get_limited_value(3., 4., 2.), 3.);
        assert_eq!(get_limited_value(5., 4., 2.), 4.);
    }
}
