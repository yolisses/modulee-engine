use crate::group::Group;
use std::{collections::HashMap, error::Error};

pub(crate) fn get_updated_group(
    group: Option<Group>,
    group_id: Option<usize>,
    new_groups: &HashMap<usize, Group>,
) -> Result<Option<Group>, Box<dyn Error>> {
    // The group is created, updated or deleted accordingly with the
    // possible corresponding new group
    if let Some(group_id) = group_id {
        if let Some(new_group) = new_groups.get(&group_id) {
            if let Some(mut group) = group {
                group.update(new_group)?;
                Ok(Some(group))
            } else {
                let mut group = new_group.clone();
                group.sort_nodes_topologically()?;
                Ok(Some(group))
            }
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
