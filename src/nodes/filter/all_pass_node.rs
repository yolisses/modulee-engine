use crate::{
    filter::all_pass::AllPass, node_trait::NodeTrait, sort::has_id::HasId, values_by_id::ValuesById,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct InputIds {
    input: usize,
    frequency: usize,
    resonance: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct AllPassNode {
    id: usize,
    input_ids: InputIds,
    #[serde(skip)]
    all_pass: AllPass,
}

impl NodeTrait for AllPassNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        let input = node_values[&self.input_ids.input];
        let frequency = node_values[&self.input_ids.frequency];
        let resonance = node_values[&self.input_ids.resonance];
        self.all_pass.process(input, frequency, resonance)
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![
            self.input_ids.input,
            self.input_ids.frequency,
            self.input_ids.resonance,
        ]
    }

    fn update(&mut self, new_node: &Self) {
        self.input_ids = new_node.input_ids.clone();
    }
}

impl HasId for AllPassNode {
    fn get_id(&self) -> usize {
        self.id
    }
}
