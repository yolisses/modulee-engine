use crate::{
    module::module::Module,
    sort::{has_id::HasId, inputs_mapping::InputsMapping},
};
use nohash_hasher::IntMap;

pub(crate) fn get_module_dependency_map(modules: &Vec<Module>) -> InputsMapping {
    let mut inputs_mapping: InputsMapping = IntMap::default();

    for module in modules {
        inputs_mapping.insert(module.get_id(), module.get_input_ids());
    }

    inputs_mapping
}
