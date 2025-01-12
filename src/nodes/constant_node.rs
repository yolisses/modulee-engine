use crate::{node_trait::NodeTrait, node_values::NodeValues};

#[derive(Debug)]
pub(crate) struct Extras {
    pub(crate) value: f64,
}

#[derive(Debug)]
pub(crate) struct ConstantNode {
    pub(crate) id: usize,
    pub(crate) extras: Extras,
}

impl ConstantNode {
    pub(crate) fn new(id: usize, value: f64) -> Self {
        Self {
            id,
            extras: Extras { value },
        }
    }
}

impl NodeTrait for ConstantNode {
    fn process(&mut self, _node_values: &NodeValues) -> f64 {
        self.extras.value
    }

    fn get_id(&self) -> usize {
        self.id
    }
}
