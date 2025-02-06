use super::inputs_mapping::InputsMapping;

pub(crate) fn validate_inputs_mapping(inputs_mapping: &InputsMapping) -> Result<(), String> {
    let mut missing_ids = vec![];
    for (key, inputs) in inputs_mapping {
        for input in inputs {
            if !inputs_mapping.contains_key(input) {
                missing_ids.push(input.clone());
            }
        }
    }

    if missing_ids.is_empty() {
        Ok(())
    } else {
        Err(format!("Missing ids for inputs: {:?}", missing_ids))
    }
}
