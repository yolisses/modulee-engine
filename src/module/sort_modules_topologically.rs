use super::get_module_dependency_map::get_module_dependency_map;
use crate::{
    module::module::Module,
    sort::{
        has_id::HasId, sort_topologically::sort_topologically,
        validate_dependency_map::validate_dependency_map,
    },
};

pub(crate) fn sort_modules_topologically(modules: &mut Vec<Module>) -> Result<(), String> {
    let dependency_map = get_module_dependency_map(modules);
    validate_dependency_map(&dependency_map)?;
    let modules_order = sort_topologically(&dependency_map);
    modules.sort_by_key(|module| modules_order.iter().position(|id| *id == module.get_id()));
    Ok(())
}
