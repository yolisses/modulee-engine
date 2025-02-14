use crate::sort::has_id::HasId;
use nohash_hasher::IntMap;

pub(crate) fn get_items_by_id<T: HasId>(items: Vec<T>) -> IntMap<usize, T> {
    let mut map = IntMap::default();
    for item in items {
        map.insert(item.get_id(), item);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct TestItem {
        id: usize,
        value: String,
    }

    impl HasId for TestItem {
        fn get_id(&self) -> usize {
            self.id
        }
    }

    #[test]
    fn test_get_items_by_id_empty() {
        let items: Vec<TestItem> = vec![];
        let result = get_items_by_id(items);
        assert!(result.is_empty());
    }

    #[test]
    fn test_get_items_by_id_single_item() {
        let items = vec![TestItem {
            id: 1,
            value: "Item 1".to_string(),
        }];
        let result = get_items_by_id(items);
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[&1],
            TestItem {
                id: 1,
                value: "Item 1".to_string()
            }
        );
    }

    #[test]
    fn test_get_items_by_id_multiple_items() {
        let items = vec![
            TestItem {
                id: 1,
                value: "Item 1".to_string(),
            },
            TestItem {
                id: 2,
                value: "Item 2".to_string(),
            },
            TestItem {
                id: 3,
                value: "Item 3".to_string(),
            },
        ];
        let result = get_items_by_id(items);
        assert_eq!(result.len(), 3);
        assert_eq!(
            result[&1],
            TestItem {
                id: 1,
                value: "Item 1".to_string()
            }
        );
        assert_eq!(
            result[&2],
            TestItem {
                id: 2,
                value: "Item 2".to_string()
            }
        );
        assert_eq!(
            result[&3],
            TestItem {
                id: 3,
                value: "Item 3".to_string()
            }
        );
    }
}
