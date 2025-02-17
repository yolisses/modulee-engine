use crate::group::Group;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct GraphData {
    pub(crate) groups: Vec<Group>,
    pub(crate) main_group_id: Option<usize>,
}
