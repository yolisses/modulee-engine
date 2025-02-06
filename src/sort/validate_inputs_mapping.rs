use super::inputs_mapping::InputsMapping;

pub(crate) fn validate_inputs_mapping(inputs_mapping: &InputsMapping) -> Result<(), String> {
    let mut missing_mappings = vec![];

    for (key, inputs) in inputs_mapping {
        for input in inputs {
            if !inputs_mapping.contains_key(input) {
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
