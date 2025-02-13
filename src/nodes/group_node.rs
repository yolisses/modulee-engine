use crate::{group::Group, node_trait::NodeTrait, sort::has_id::HasId, values_by_id::ValuesById};
use serde::Deserialize;
use std::{collections::HashMap, error::Error};

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Extras {
    input_target_ids: HashMap<usize, usize>,
}

/// Returns the phase value between 0 and 1 given a time and a frequency
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct GroupNode {
    id: usize,
    extras: Extras,
    #[serde(skip)]
    group: Group,
}

impl GroupNode {
    pub(crate) fn update_groups(
        &mut self,
        new_groups: &HashMap<usize, Group>,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(new_group) = new_groups.get(&self.group.get_id()) {
            self.group.update(new_group)?;
        } else {
            panic!(
                "Can't find a group with id {} to update group node with id {}",
                self.group.get_id(),
                self.get_id()
            )
        }
        Ok(())
    }
}

impl NodeTrait for GroupNode {
    fn process(&mut self, node_values: &ValuesById) -> f32 {
        self.group
            .update_input_nodes(node_values, &self.extras.input_target_ids);
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
