use crate::{node_trait::NodeTrait, sort::has_id::HasId, values_by_id::ValuesById};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct GateNode {
    id: usize,
    #[serde(skip)]
    is_active: bool,
}

impl GateNode {
    pub(crate) fn set_is_active(&mut self, is_active: bool) {
        self.is_active = is_active
    }
}

impl NodeTrait for GateNode {
    fn process(&mut self, node_values: &mut ValuesById) {
        let value = if self.is_active { 1. } else { 0. };
        node_values.insert(self.id, value);
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![]
    }
}

impl HasId for GateNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
