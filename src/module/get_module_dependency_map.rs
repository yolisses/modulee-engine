use crate::{
    module::module::Module,
    sort::{dependency_map::DependencyMap, has_id::HasId},
};
use vector_map::VecMap;

pub(crate) fn get_module_dependency_map(modules: &Vec<Module>) -> DependencyMap {
    let mut dependency_map: DependencyMap = VecMap::default();

    for module in modules {
        dependency_map.insert(module.get_id(), module.get_child_module_ids());
    }

    dependency_map
}
