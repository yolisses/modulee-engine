pub(crate) fn get_bool_value(value: f32) -> bool {
    value > 0.5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bool_value_true() {
        assert!(get_bool_value(0.6));
    }

    #[test]
    fn test_get_bool_value_false() {
        assert!(!get_bool_value(0.4));
    }

    #[test]
    fn test_get_bool_value_boundary_false() {
        assert!(!get_bool_value(0.5));
    }
}
