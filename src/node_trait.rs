use crate::node_values::NodeValues;

pub(crate) trait NodeTrait {
    fn get_id(&self) -> usize;
    fn get_input_ids(&self) -> Vec<usize>;
    fn process(&mut self, node_values: &NodeValues) -> f32;
}
