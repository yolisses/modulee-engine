use crate::{declare_get_id, declare_get_input_ids_and_its_getter, declare_update};
use crate::{node_trait::NodeTrait, values_by_id::ValuesById};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct DivideNode {
    id: usize,
    input_ids: InputIds,
}

declare_get_id! {DivideNode}
declare_update! {DivideNode}
declare_get_input_ids_and_its_getter! {DivideNode, input1, input2}

impl NodeTrait for DivideNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        let input1 = node_values[&self.input_ids.input1];
        let input2 = node_values[&self.input_ids.input2];
        input1 / input2
    }
}
