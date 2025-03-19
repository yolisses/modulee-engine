use crate::module::Module;
use nohash_hasher::IntMap;
use std::error::Error;

pub(crate) fn get_updated_module(
    module: Option<Module>,
    module_id: Option<usize>,
    new_modules: &IntMap<usize, Module>,
) -> Result<Option<Module>, Box<dyn Error>> {
    // The module is created, updated or deleted accordingly with the
    // possible corresponding new module
    if let Some(module_id) = module_id {
        if let Some(new_module) = new_modules.get(&module_id) {
            if let Some(mut module) = module {
                module.update(new_module)?;
                Ok(Some(module))
            } else {
                let mut module = new_module.clone();
                module.sort_nodes_topologically()?;
                Ok(Some(module))
            }
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
