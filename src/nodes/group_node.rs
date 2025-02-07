use crate::{graph::Graph, node_trait::NodeTrait, sort::has_id::HasId, values_by_id::ValuesById};
use serde::Deserialize;

/// Returns the phase value between 0 and 1 given a time and a frequency
#[derive(Debug, Deserialize)]
pub(crate) struct GroupNode {
    id: usize,
    #[serde(skip)]
    graph: Graph,
}

impl NodeTrait for GroupNode {
    fn process(&mut self, node_values: &mut ValuesById) {
        self.graph.process();
        // TODO use all outputs
        let value = self.graph.get_output_value();
        node_values.insert(self.id, value);
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
