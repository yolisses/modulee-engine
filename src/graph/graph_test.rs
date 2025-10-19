#[cfg(test)]
mod tests {
    use crate::{sort::has_id::HasId, Graph};
    use std::{fs, ops::Index};

    #[test]
    fn test_update_modules() {
        let mut graph = Graph::new(1.);

        graph
            .update_from_json(r#"{"main_module_id": 10, "modules": [{"id": 10, "nodes": []}]}"#)
            .unwrap();

        assert_eq!(
            graph
                .main_module_instances
                .as_ref()
                .unwrap()
                .index(0)
                .get_id(),
            10
        );

        graph
            .update_from_json(r#"{"main_module_id": 20, "modules": [{"id": 10, "nodes": []}, {"id": 20, "nodes": []}]}"#)
            .unwrap();

        assert_eq!(
            graph
                .main_module_instances
                .as_ref()
                .unwrap()
                .index(0)
                .get_id(),
            20
        );
    }

    #[test]
    fn test_load_graph_from_file() {
        let data_path = std::path::Path::new("src/graph/test/graphEngineData.json");
        let data = fs::read_to_string(&data_path).expect("Can't find the file");
        let mut graph = Graph::new(48000.);
        graph
            .update_from_json(&data)
            .expect("update_from_json failed");
        graph.process();
        graph.set_note_on(60.);
        graph.process();
    }
}
