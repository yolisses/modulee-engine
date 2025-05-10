use crate::sort::dependency_map::DependencyMap;

pub(crate) fn create_inputs_mapping(data: &[(usize, Vec<usize>)]) -> DependencyMap {
    let mut inputs_mapping = DependencyMap::default();
    for (key, value) in data {
        inputs_mapping.insert(*key, value.clone());
    }
    inputs_mapping
}
