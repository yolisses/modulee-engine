use crate::{
    module::module::Module,
    sort::{dependency_map::DependencyMap, has_id::HasId},
};
use nohash_hasher::IntMap;

pub(crate) fn get_module_dependency_map(modules: &Vec<Module>) -> DependencyMap {
    let mut inputs_mapping: DependencyMap = IntMap::default();

    for module in modules {
        inputs_mapping.insert(module.get_id(), module.get_input_ids());
    }

    inputs_mapping
}
