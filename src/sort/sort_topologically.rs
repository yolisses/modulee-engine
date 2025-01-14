use super::{get_items_by_id::get_items_by_id, has_id::HasId, items_by_id::ItemsById};
use crate::{node::Node, node_trait::NodeTrait};
use std::collections::HashMap;
type Shifts = HashMap<usize, usize>;

fn update_shifts(
    shifts: &mut Shifts,
    nodes_by_id: &ItemsById<Node>,
    node_id: usize,
    counter: usize,
) -> usize {
    let mut counter = counter + 1;
    shifts.insert(node_id, counter);
    match nodes_by_id.get(&node_id) {
        Some(node) => {
            for input_node_id in node.get_input_ids() {
                counter = update_shifts(shifts, nodes_by_id, input_node_id, counter)
            }
        }
        None => panic!("Node not found with id {}", node_id),
    }
    counter
}

// pub(crate) fn sort_topologically(nodes: Vec<Node>) -> Vec<Node> {
//     let node_ids: Vec<usize> = nodes.into_iter().map(|node| node.get_id()).collect();

//     let nodes_by_id = get_items_by_id(nodes);
//     let mut shifts: HashMap<usize, usize> = HashMap::new();

//     let mut counter = 0;
//     for node_id in node_ids {
//         counter = update_shifts(&mut shifts, &nodes_by_id, node_id, counter)
//     }

//     // Return nodes sorted by shift
//     nodes
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_shifts() {
        let mut shifts: Shifts = HashMap::new();
        shifts.insert(1, 1);
        shifts.insert(2, 2);
        shifts.insert(3, 3);
    }
}
