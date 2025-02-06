use crate::{graph::Graph, node_trait::NodeTrait, sort::has_id::HasId, values_by_id::ValuesById};
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct InputIds {
    frequency: usize,
}

/// Returns the phase value between 0 and 1 given a time and a frequency
#[derive(Debug, Deserialize)]
pub(crate) struct GroupNode {
    id: usize,
    input_ids: InputIds,
    #[serde(skip)]
    graph: Graph,
}

impl NodeTrait for GroupNode {
    fn process(&mut self, _node_values: &ValuesById) -> f32 {
        todo!()
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![self.input_ids.frequency]
    }
}

impl HasId for GroupNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
