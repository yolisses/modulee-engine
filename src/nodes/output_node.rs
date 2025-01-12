use crate::{node_trait::NodeTrait, node_values::NodeValues};

#[derive(Debug)]
pub(crate) struct InputIds {
    pub(crate) input: usize,
}

/// Saves the input value in a accessible way
#[derive(Debug)]
pub(crate) struct OutputNode {
    value: f64,
    pub(crate) id: usize,
    pub(crate) input_ids: InputIds,
}

impl OutputNode {
    pub(crate) fn new(id: usize, input: usize) -> Self {
        Self {
            id,
            value: 0.,
            input_ids: InputIds { input },
        }
    }

    pub(crate) fn get_value(&self) -> f64 {
        self.value
    }
}

impl NodeTrait for OutputNode {
    fn process(&mut self, node_values: &NodeValues) -> f64 {
        let input = node_values.get(&self.input_ids.input).unwrap();
        self.value = *input;
        self.value
    }

    fn get_id(&self) -> usize {
        self.id
    }
}
