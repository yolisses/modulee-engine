use crate::module::{module::Module, sort_modules_topologically::sort_modules_topologically};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct GraphData {
    pub(crate) modules: Vec<Module>,
    pub(crate) main_module_id: Option<usize>,
}

impl GraphData {
    pub(crate) fn prepare(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for module in &mut self.modules {
            module.prepare_nodes();
        }

        sort_modules_topologically(&mut self.modules)?;

        // TODO allow arbitrarily deep preparation
        let module_options = &self.modules.clone();
        for module in &mut self.modules {
            module.prepare_modules_in_nodes(module_options);
        }
        Ok(())
    }
}
