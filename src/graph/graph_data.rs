use crate::module::{module::Module, sort_modules_topologically::sort_modules_topologically};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct GraphData {
    pub(crate) modules: Vec<Module>,
    pub(crate) main_module_id: Option<usize>,
}

impl GraphData {
    pub(crate) fn prepare(&mut self, sample_rate: f32) -> Result<(), Box<dyn std::error::Error>> {
        for module in &mut self.modules {
            module.prepare_nodes(sample_rate);
        }

        sort_modules_topologically(&mut self.modules)?;

        // Iterate over indices to avoid mutable borrow conflict
        for i in 0..self.modules.len() {
            let modules = self.modules.clone();
            let module = &mut self.modules[i];
            module.prepare_modules_in_nodes(&modules);
        }
        Ok(())
    }
}
