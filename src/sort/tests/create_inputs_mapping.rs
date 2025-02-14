use crate::sort::inputs_mapping::InputsMapping;

pub(crate) fn create_inputs_mapping(data: &[(usize, Vec<usize>)]) -> InputsMapping {
    let mut inputs_mapping = InputsMapping::default();
    for (key, value) in data {
        inputs_mapping.insert(*key, value.clone());
    }
    inputs_mapping
}
