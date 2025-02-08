use crate::{group::Group, node_trait::NodeTrait, sort::has_id::HasId, values_by_id::ValuesById};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub(crate) struct Extras {
    input_target_ids: HashMap<usize, usize>,
}

/// Returns the phase value between 0 and 1 given a time and a frequency
#[derive(Debug, Deserialize)]
pub(crate) struct GroupNode {
    id: usize,
    extras: Extras,
    #[serde(skip)]
    group: Group,
}

impl NodeTrait for GroupNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        // Set input nodes
        let mut input_values = HashMap::new();
        for (input_id, target_id) in &self.extras.input_target_ids {
            let target_value = node_values[target_id];
            input_values.insert(*input_id, target_value);
        }
        self.group.update_input_nodes(input_values);

        self.group.process();
        // TODO use all outputs
        self.group.get_output_value()
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![]
    }
}

impl HasId for GroupNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
