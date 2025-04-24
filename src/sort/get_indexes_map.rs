use super::node_indexes::NodeIndexes;

pub(crate) fn get_indexes_map(values: Vec<usize>) -> NodeIndexes {
    let mut result: NodeIndexes = Default::default();
    for (index, value) in values.iter().enumerate() {
        result.insert(*value, index);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_indexes_map_empty() {
        let values = vec![];
        let result = get_indexes_map(values);
        assert_eq!(result, NodeIndexes::default());
    }

    #[test]
    fn test_get_indexes_map_single_value() {
        let values = vec![42];
        let mut expected = NodeIndexes::default();
        expected.insert(42, 0);
        let result = get_indexes_map(values);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_indexes_map_multiple_values() {
        let values = vec![10, 20, 30];
        let mut expected = NodeIndexes::default();
        expected.insert(10, 0);
        expected.insert(20, 1);
        expected.insert(30, 2);
        let result = get_indexes_map(values);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_indexes_map_duplicate_values() {
        let values = vec![5, 5, 10];
        let mut expected = NodeIndexes::default();
        expected.insert(5, 1); // Last occurrence of 5
        expected.insert(10, 2);
        let result = get_indexes_map(values);
        assert_eq!(result, expected);
    }
}
