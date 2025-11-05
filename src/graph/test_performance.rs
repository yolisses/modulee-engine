#[cfg(test)]
mod tests {
    use crate::Graph;
    use std::fs;

    #[test]
    fn test_performance() {
        let data_path = std::path::Path::new("benches/piano_benchmark.json");
        let data = fs::read_to_string(&data_path).expect("Can't find the file");
        let mut graph = Graph::new(48000.);
        graph
            .update_from_json(&data)
            .expect("update_from_json failed");
        graph.process();
        graph.set_note_on(60.);
        graph.set_note_on(62.);
        graph.set_note_on(64.);

        let seconds = 100;
        let sample_rate = 48000;
        let samples = seconds * sample_rate;
        for _ in 0..samples {
            graph.process();
        }
    }
}
