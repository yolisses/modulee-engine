use crate::sort::dependency_map::DependencyMap;

pub(crate) fn create_dependency_map(data: &[(usize, Vec<usize>)]) -> DependencyMap {
    let mut dependency_map = DependencyMap::default();
    for (key, value) in data {
        dependency_map.insert(*key, value.clone());
    }
    dependency_map
}
