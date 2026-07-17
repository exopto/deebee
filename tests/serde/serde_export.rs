use deebee::{Graph, Arrow};

#[test]
fn export_graph_formats() {
    let mut graph = Graph::new();
    let a_id = graph.add("Root Node");
    let b_id = graph.add("Child Node");
    graph.connect(a_id, b_id, &Arrow::CHILDREN).unwrap();

    // The problem is that Node's internal `linked` is a HashMap<Arrow, HashSet<Uuid>>.
    // serde_json/json5/toml cannot handle Arrow as a key because it's not a string.
    // To export the literal graph, we must convert these internal maps to vectors of tuples.
    
    #[derive(serde::Serialize)]
    struct NodeExport<'a> {
        id: String,
        data: &'a deebee::Value,
        linked: Vec<(&'a deebee::Arrow, Vec<String>)>,
    }

    let mut nodes_export = Vec::new();
    for (id, node) in graph.to_map() {
        let linked = node.arrows()
            .map(|(arrow, ids)| (arrow, ids.iter().map(|i| i.to_string()).collect()))
            .collect();
        
        nodes_export.push(NodeExport {
            id: id.to_string(),
            data: &node.data,
            linked,
        });
    }

    // 1. serde_json
    let json = serde_json::to_string_pretty(&nodes_export).expect("JSON serialization failed");
    std::fs::write(".local/graph_serde_json.json", json).expect("Failed to write JSON");

    // 2. json5
    let j5 = json5::to_string(&nodes_export).expect("JSON5 serialization failed");
    std::fs::write(".local/graph_serde_json5.json5", j5).expect("Failed to write JSON5");

    // 3. toml
    #[derive(serde::Serialize)]
    struct TomlWrapper<'a> {
        nodes: Vec<NodeExport<'a>>,
    }
    let toml_data = TomlWrapper { nodes: nodes_export };
    let tml = toml::to_string(&toml_data).expect("TOML serialization failed");
    std::fs::write(".local/graph_serde_toml.toml", tml).expect("Failed to write TOML");

    // 4. bincode
    // Bincode is binary and handles the actual Graph struct (HashMap keys etc) perfectly.
    let bin = bincode::serialize(&graph).expect("Bincode serialization failed");
    std::fs::write(".local/graph_serde_bin.bin", bin).expect("Failed to write Bincode");
}
