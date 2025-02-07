use crate::{node_trait::NodeTrait, sort::has_id::HasId, values_by_id::ValuesById};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Extras {
    value: f32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ConstantNode {
    id: usize,
    extras: Extras,
}

impl NodeTrait for ConstantNode {
    fn process(&mut self, node_values: &mut ValuesById) {
        let value = self.extras.value;
        node_values.insert(self.id, value);
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![]
    }
}

impl HasId for ConstantNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
