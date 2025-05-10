use super::dependency_map::DependencyMap;

pub(crate) fn validate_dependency_map(dependency_map: &DependencyMap) -> Result<(), String> {
    let mut missing_mappings = vec![];

    for (key, inputs) in dependency_map {
        for input in inputs {
            if !dependency_map.contains_key(input) {
                missing_mappings.push(vec![*key, *input]);
            }
        }
    }

    if missing_mappings.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "Missing ids for inputs (node id, requested node id): {:?}",
            missing_mappings
        ))
    }
}
