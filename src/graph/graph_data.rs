use crate::module::module::Module;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct GraphData {
    pub(crate) modules: Vec<Module>,
    pub(crate) main_module_id: Option<usize>,
}

impl GraphData {
    pub(crate) fn prepare(&mut self) {
        for module in &mut self.modules {
            module.prepare();
        }
    }
}
