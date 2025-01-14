use super::{has_id::HasId, items_by_id::ItemsById};
use std::collections::HashMap;

pub(crate) fn get_items_by_id<T: HasId>(nodes: Vec<T>) -> ItemsById<T> {
    let mut nodes_by_id: ItemsById<T> = HashMap::new();
    for node in nodes {
        nodes_by_id.insert(node.get_id(), node);
    }
    nodes_by_id
}

#[cfg(test)]
mod tests {
    use crate::sort::has_id::HasId;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[derive(Debug, PartialEq)]
    struct MockItem {
        id: usize,
    }

    impl MockItem {
        fn new(id: usize) -> Self {
            Self { id }
        }
    }

    impl HasId for MockItem {
        fn get_id(&self) -> usize {
            self.id
        }
    }

    #[test]
    fn test_get_items_by_id() {
        let items: Vec<MockItem> = vec![MockItem::new(1), MockItem::new(2), MockItem::new(3)];

        let expected_result: ItemsById<MockItem> = HashMap::from([
            (1, MockItem::new(1)),
            (2, MockItem::new(2)),
            (3, MockItem::new(3)),
        ]);

        assert_eq!(get_items_by_id(items), expected_result);
    }
}
