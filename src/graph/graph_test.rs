#[cfg(test)]
mod tests {
    use crate::{sort::has_id::HasId, Graph};

    #[test]
    fn test_update_modules() {
        let mut graph = Graph::default();

        graph
            .update_from_json(r#"{"main_module_id": 10, "modules": [{"id": 10, "nodes": []}]}"#)
            .unwrap();

        assert_eq!(graph.main_module.as_ref().unwrap().get_id(), 10);

        graph
            .update_from_json(r#"{"main_module_id": 20, "modules": [{"id": 10, "nodes": []}, {"id": 20, "nodes": []}]}"#)
            .unwrap();

        assert_eq!(graph.main_module.as_ref().unwrap().get_id(), 20);
    }
}
