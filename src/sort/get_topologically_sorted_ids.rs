use super::{inputs_mapping::InputsMapping, shifts::Shifts, update_shifts::update_shifts};

pub(crate) fn get_topologically_sorted_ids(inputs_mapping: &InputsMapping) -> Vec<usize> {
    let mut counter = 0;
    let mut shifts: Shifts = Shifts::new();

    // Sort nodes_ids to avoid unpredictable behaviors due to HashMap random
    // keys order
    let mut node_ids: Vec<_> = inputs_mapping.keys().cloned().collect();
    node_ids.sort();
    println!("{:?}", node_ids);
    for node_id in node_ids {
        update_shifts(node_id, &mut shifts, &mut counter, &inputs_mapping);
    }

    println!("{:?}", shifts);
    let mut sorted_ids: Vec<_> = shifts.keys().cloned().collect();
    sorted_ids.sort_by_key(|id| shifts[id]);
    sorted_ids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_topologically_sorted_ids_without_inputs() {
        let inputs_mapping: InputsMapping =
            InputsMapping::from([(1, vec![]), (2, vec![3]), (3, vec![])]);
        let topologically_sorted_ids = get_topologically_sorted_ids(&inputs_mapping);

        println!("{:?}", inputs_mapping);

        /*
        1
        2
            3
        3
        */

        assert_eq!(topologically_sorted_ids, vec![1, 2, 3]);
    }

    #[test]
    fn test_get_topologically_sorted_ids_directly() {
        let inputs_mapping: InputsMapping = InputsMapping::from([
            (1, vec![2, 3]),
            (2, vec![5]),
            (3, vec![]),
            (4, vec![]),
            (5, vec![]),
        ]);

        let topologically_sorted_ids = get_topologically_sorted_ids(&inputs_mapping);

        /*
        1
            2
                5
            3
        4
        */

        assert_eq!(topologically_sorted_ids, vec![1, 2, 5, 3, 4]);
    }

    #[test]
    fn test_get_topologically_sorted_ids_with_recalculation() {
        let inputs_mapping: InputsMapping = InputsMapping::from([
            (1, vec![2, 3]),
            (2, vec![5]),
            (3, vec![5]),
            (4, vec![]),
            (5, vec![]),
        ]);

        let topologically_sorted_ids = get_topologically_sorted_ids(&inputs_mapping);

        /*
        1 -> 1
            2 -> 2
                5 -> 3
            3 -> 4
                5 -> 5
        */

        assert_eq!(topologically_sorted_ids, vec![1, 2, 3, 5]);
    }

    #[test]
    fn test_get_topologically_sorted_ids_with_long_sequence() {
        let inputs_mapping: InputsMapping = InputsMapping::from([
            (2, vec![]),
            (3, vec![4, 7]),
            (7, vec![1, 6]),
            (5, vec![2]),
            (6, vec![2]),
            (1, vec![]),
            (4, vec![5, 2]),
        ]);

        let topologically_sorted_ids = get_topologically_sorted_ids(&inputs_mapping);

        /*
        3 -> 1
            4 -> 2
                5 -> 3
                    2 -> 4
                2
            7 -> 5
                1 -> 6
                6 -> 7
                    2 -> 8
        */

        assert_eq!(topologically_sorted_ids, vec![3, 4, 5, 7, 1, 6, 2]);
    }
}
