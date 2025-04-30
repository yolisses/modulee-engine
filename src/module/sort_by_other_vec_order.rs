use crate::sort::has_id::HasId;

pub(crate) fn sort_by_other_vec_order(items: &mut [impl HasId], order_items: &[impl HasId]) {
    let id_order: Vec<usize> = order_items.iter().map(|item| item.get_id()).collect();
    items.sort_by_key(|item| {
        id_order
            .iter()
            .position(|&id| id == item.get_id())
            .unwrap_or(usize::MAX)
    });
}

#[cfg(test)]
mod tests {
    use crate::{module::sort_by_other_vec_order::sort_by_other_vec_order, sort::has_id::HasId};

    #[derive(Debug, PartialEq)]
    struct Item {
        id: usize,
        value: u32,
    }

    impl HasId for Item {
        fn get_id(&self) -> usize {
            self.id
        }
    }

    #[test]
    fn test_sort_by_order_normal() {
        let mut items = vec![
            Item { id: 3, value: 30 },
            Item { id: 1, value: 10 },
            Item { id: 2, value: 20 },
        ];
        let order_items = vec![
            Item { id: 1, value: 100 },
            Item { id: 2, value: 200 },
            Item { id: 3, value: 300 },
        ];

        sort_by_other_vec_order(&mut items, &order_items);

        let expected = vec![
            Item { id: 1, value: 10 },
            Item { id: 2, value: 20 },
            Item { id: 3, value: 30 },
        ];
        assert_eq!(items, expected);
    }

    #[test]
    fn test_sort_by_order_empty_items() {
        let mut items: Vec<Item> = vec![];
        let order_items: Vec<Item> = vec![];

        sort_by_other_vec_order(&mut items, &order_items);

        assert_eq!(items, vec![] as Vec<Item>);
    }
}
