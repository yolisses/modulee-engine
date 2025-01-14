use super::inputs_mapping::InputsMapping;
use std::collections::HashMap;

type Shifts = HashMap<usize, usize>;

fn update_shifts(
    node_id: usize,
    shifts: &mut Shifts,
    counter: &mut usize,
    inputs_mapping: &mut InputsMapping,
) {
    *counter += 1;
    shifts.insert(node_id, *counter);
    let input_ids = inputs_mapping[&node_id].clone();
    for input_id in input_ids {
        update_shifts(input_id, shifts, counter, inputs_mapping);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_shifts_without_inputs() {
        let node_id = 1;
        let mut counter = 0;
        let mut shifts: Shifts = HashMap::new();
        let mut inputs_mapping: InputsMapping =
            HashMap::from([(1, vec![]), (2, vec![3]), (3, vec![])]);

        update_shifts(node_id, &mut shifts, &mut counter, &mut inputs_mapping);

        /*
        1 -> 1
        */

        assert_eq!(shifts, HashMap::from([(1, 1)]))
    }

    #[test]
    fn test_update_shifts_directly() {
        let node_id = 1;
        let mut counter = 0;
        let mut shifts: Shifts = HashMap::new();
        let mut inputs_mapping: InputsMapping = HashMap::from([
            (1, vec![2, 3]),
            (2, vec![5]),
            (3, vec![]),
            (4, vec![]),
            (5, vec![]),
        ]);

        update_shifts(node_id, &mut shifts, &mut counter, &mut inputs_mapping);

        /*
        1 -> 1
            2 -> 2
                5 -> 3
            3 -> 4
        */

        assert_eq!(shifts, HashMap::from([(1, 1), (2, 2), (5, 3), (3, 4),]))
    }

    #[test]
    fn test_update_shifts_with_recalculation() {
        let node_id = 1;
        let mut counter = 0;
        let mut shifts: Shifts = HashMap::new();
        let mut inputs_mapping: InputsMapping = HashMap::from([
            (1, vec![2, 3]),
            (2, vec![5]),
            (3, vec![5]),
            (4, vec![]),
            (5, vec![]),
        ]);

        update_shifts(node_id, &mut shifts, &mut counter, &mut inputs_mapping);

        /*
        1 -> 1
            2 -> 2
                5 -> 3
            3 -> 4
                5 -> 5
        */

        assert_eq!(shifts, HashMap::from([(1, 1), (2, 2), (3, 4), (5, 5),]))
    }

    #[test]
    fn test_update_shifts_with_long_sequence() {
        let node_id = 3;
        let mut counter = 0;
        let mut shifts: Shifts = HashMap::new();
        let mut inputs_mapping: InputsMapping = HashMap::from([
            (2, vec![]),
            (3, vec![4, 7]),
            (7, vec![1, 6]),
            (5, vec![2]),
            (6, vec![2]),
            (1, vec![]),
            (4, vec![5, 2]),
        ]);

        update_shifts(node_id, &mut shifts, &mut counter, &mut inputs_mapping);

        /*
        3 -> 1
            4 -> 2
                5 -> 3
                    2 -> 4
                2 -> 5
            7 -> 6
                1 -> 7
                6 -> 8
                    2 -> 9
        */

        assert_eq!(
            shifts,
            HashMap::from([(3, 1), (4, 2), (5, 3), (7, 6), (1, 7), (6, 8), (2, 9),])
        )
    }
}
