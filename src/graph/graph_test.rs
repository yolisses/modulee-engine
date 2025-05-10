#[cfg(test)]
mod tests {
    use crate::{sort::has_id::HasId, Graph};

    #[test]
    fn test_update_modules() {
        let mut graph = Graph::default();

        graph
            .update_from_json(r#"{"main_module_id": null, "modules": [{"id": 1, "nodes": []}]}"#)
            .unwrap();

        assert_eq!(
            graph.modules_by_id.keys().cloned().collect::<Vec<_>>(),
            vec![1]
        );
        let module = &graph.modules_by_id[&1];
        assert_eq!(module.get_id(), 1);

        graph
            .update_from_json(r#"{"main_module_id": null, "modules": [{"id": 2, "nodes": []}]}"#)
            .unwrap();

        assert_eq!(
            graph.modules_by_id.keys().cloned().collect::<Vec<_>>(),
            vec![2]
        );
        let module = &graph.modules_by_id[&2];
        assert_eq!(module.get_id(), 2);
    }
}
