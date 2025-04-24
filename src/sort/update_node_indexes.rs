use super::{inputs_mapping::InputsMapping, node_indexes::NodeIndexes};

pub(crate) fn update_node_indexes(
    node_id: usize,
    node_indexes: &mut NodeIndexes,
    counter: &mut usize,
    inputs_mapping: &InputsMapping,
) {
    if node_indexes.contains_key(&node_id) {
        return;
    }

    let input_ids = inputs_mapping[&node_id].clone();
    for input_id in input_ids {
        update_node_indexes(input_id, node_indexes, counter, inputs_mapping);
    }

    *counter += 1;
    node_indexes.insert(node_id, *counter);
}

#[cfg(test)]
mod tests {
    use crate::sort::tests::{
        create_inputs_mapping::create_inputs_mapping, create_nodes_indexes::create_nodes_indexes,
    };

    use super::*;

    #[test]
    fn test_update_node_indexes_without_inputs() {
        let node_id = 1;
        let mut counter = 0;
        let mut node_indexes = create_nodes_indexes(&[]);
        let inputs_mapping = create_inputs_mapping(&[(1, vec![]), (2, vec![3]), (3, vec![])]);

        update_node_indexes(node_id, &mut node_indexes, &mut counter, &inputs_mapping);

        /*
        1 -> 1
        */

        assert_eq!(node_indexes, create_nodes_indexes(&[(1, 1)]))
    }

    #[test]
    fn test_update_node_indexes_directly() {
        let node_id = 1;
        let mut counter = 0;
        let mut node_indexes = create_nodes_indexes(&[]);
        let inputs_mapping = create_inputs_mapping(&[
            (1, vec![2, 3]),
            (2, vec![5]),
            (3, vec![]),
            (4, vec![]),
            (5, vec![]),
        ]);

        update_node_indexes(node_id, &mut node_indexes, &mut counter, &inputs_mapping);

        /*
        1 -> 4
            2 -> 2
                5 -> 1
            3 -> 3
        */

        assert_eq!(
            node_indexes,
            create_nodes_indexes(&[(1, 4), (2, 2), (5, 1), (3, 3)])
        )
    }

    #[test]
    fn test_update_node_indexes_with_recalculation() {
        let node_id = 1;
        let mut counter = 0;
        let mut node_indexes = create_nodes_indexes(&[]);
        let inputs_mapping = create_inputs_mapping(&[
            (1, vec![2, 3]),
            (2, vec![5]),
            (3, vec![5]),
            (4, vec![]),
            (5, vec![]),
        ]);

        update_node_indexes(node_id, &mut node_indexes, &mut counter, &inputs_mapping);

        /*
        1 -> 4
            2 -> 2
                5 -> 1
            3 -> 3
                5
        */

        assert_eq!(
            node_indexes,
            create_nodes_indexes(&[(5, 1), (2, 2), (3, 3), (1, 4)])
        )
    }

    #[test]
    fn test_update_node_indexes_with_long_sequence() {
        let node_id = 3;
        let mut counter = 0;
        let mut node_indexes = create_nodes_indexes(&[]);
        let inputs_mapping = create_inputs_mapping(&[
            (2, vec![]),
            (3, vec![4, 7]),
            (7, vec![1, 6]),
            (5, vec![2]),
            (6, vec![2]),
            (1, vec![]),
            (4, vec![5, 2]),
        ]);

        update_node_indexes(node_id, &mut node_indexes, &mut counter, &inputs_mapping);

        /*
        3 -> 7
            4 -> 3
                5 -> 2
                    2 -> 1
                2
            7 -> 6
                1 -> 4
                6 -> 5
                    2
        */

        assert_eq!(
            node_indexes,
            create_nodes_indexes(&[(2, 1), (5, 2), (4, 3), (1, 4), (6, 5), (7, 6), (3, 7)])
        )
    }
}
