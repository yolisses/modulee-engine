use assert_approx_eq::assert_approx_eq;

pub(crate) fn relative_eq_array(actual: Vec<f32>, expected: Vec<f32>) {
    assert_eq!(actual.len(), expected.len());
    for (actual_value, expected_value) in actual.iter().zip(expected.iter()) {
        assert_approx_eq!(actual_value, expected_value);
    }
}
