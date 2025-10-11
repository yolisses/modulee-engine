#[cfg(test)]
mod tests {
    use crate::{sort::has_id::HasId, Graph};
    use std::fs;

    #[test]
    fn test_update_modules() {
        let mut graph = Graph::new(1.);

        graph
            .update_from_json(r#"{"main_module_id": 10, "modules": [{"id": 10, "nodes": []}]}"#)
            .unwrap();

        assert_eq!(graph.main_module.as_ref().unwrap().get_id(), 10);

        graph
            .update_from_json(r#"{"main_module_id": 20, "modules": [{"id": 10, "nodes": []}, {"id": 20, "nodes": []}]}"#)
            .unwrap();

        assert_eq!(graph.main_module.as_ref().unwrap().get_id(), 20);
    }

    #[test]
    fn test_load_graph_from_file() {
        let data = fs::read_to_string("test/graphEngineData.json")
            .expect("failed to read test/graphEngineData.json");
        let mut graph = Graph::new(1.);
        graph
            .update_from_json(&data)
            .expect("update_from_json failed");
        assert!(graph.main_module.is_some());
        let id = graph.main_module.as_ref().unwrap().get_id();
        // sanity: id should be displayable/non-empty when stringified
        assert!(!id.to_string().is_empty());
    }

    #[test]
    fn test_update_from_file_then_inline() {
        let data = fs::read_to_string("test/graphEngineData.json")
            .expect("failed to read test/graphEngineData.json");
        let mut graph = Graph::new(1.);
        graph
            .update_from_json(&data)
            .expect("initial update failed");

        // apply a deterministic inline update and verify the main module becomes 9999
        graph
            .update_from_json(r#"{"main_module_id": 9999, "modules":[{"id":9999,"nodes":[]} ]}"#)
            .expect("inline update failed");

        assert_eq!(graph.main_module.as_ref().unwrap().get_id(), 9999);
    }
}
