use super::dependency_map::DependencyMap;
use std::collections::HashSet;

/// Performs topological sorting on a directed acyclic graph (DAG). Takes an
/// InputsMapping where keys are node IDs and values are vectors of dependent
/// node IDs. Returns a vector of node IDs in topological order. Assumes the
/// input graph has no cycles.
pub(crate) fn sort_topologically(graph: &DependencyMap) -> Vec<usize> {
    let mut result = Vec::new();
    let mut visited = HashSet::new();

    fn dfs(
        node: usize,
        graph: &DependencyMap,
        visited: &mut HashSet<usize>,
        result: &mut Vec<usize>,
    ) {
        // If node is already visited, skip it
        if visited.contains(&node) {
            return;
        }

        // Mark node as visited
        visited.insert(node);

        // Visit all neighbors
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                dfs(neighbor, graph, visited, result);
            }
        }

        // Add node to result
        result.push(node);
    }

    // Process all nodes in the graph
    for &node in graph.keys() {
        if !visited.contains(&node) {
            dfs(node, graph, &mut visited, &mut result);
        }
    }

    // Reverse the result to get correct topological order
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sort::tests::create_dependency_map::create_dependency_map;

    #[test]
    fn test_sort_topologically_1() {
        let dependency_map = create_dependency_map(&vec![
            //
            (1, vec![]),
            (2, vec![3]),
            (3, vec![]),
        ]);
        let topologically_sorted_ids = sort_topologically(&dependency_map);

        assert_eq!(topologically_sorted_ids, vec![1, 3, 2]);
    }

    #[test]
    fn test_sort_topologically_2() {
        let dependency_map = create_dependency_map(&[(1, vec![2, 3]),
            (2, vec![5]),
            (3, vec![]),
            (4, vec![]),
            (5, vec![])]);

        let topologically_sorted_ids = sort_topologically(&dependency_map);

        assert_eq!(topologically_sorted_ids, vec![5, 2, 3, 1, 4]);
    }

    #[test]
    fn test_sort_topologically_3() {
        let dependency_map = create_dependency_map(&[(1, vec![2, 3]),
            (2, vec![5]),
            (3, vec![5]),
            (4, vec![]),
            (5, vec![])]);

        let topologically_sorted_ids = sort_topologically(&dependency_map);

        assert_eq!(topologically_sorted_ids, vec![5, 2, 3, 1, 4]);
    }

    #[test]
    fn test_sort_topologically_4() {
        let dependency_map = create_dependency_map(&vec![
            (2, vec![]),
            (3, vec![4, 7]),
            (7, vec![1, 6]),
            (5, vec![2]),
            (6, vec![2]),
            (1, vec![]),
            (4, vec![5, 2]),
        ]);

        let topologically_sorted_ids = sort_topologically(&dependency_map);

        assert_eq!(topologically_sorted_ids, vec![1, 2, 5, 4, 6, 7, 3]);
    }

    #[test]
    fn test_sort_topologically_5() {
        let dependency_map = create_dependency_map(&[(0, vec![]),
            (7, vec![]),
            (8, vec![9]),
            (9, vec![0, 0])]);

        let topologically_sorted_ids = sort_topologically(&dependency_map);

        assert_eq!(topologically_sorted_ids, vec![0, 9, 8, 7]);
    }
}
