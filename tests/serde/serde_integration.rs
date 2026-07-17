use deebee::{Graph, Node, Arrow};

#[test]
fn test_graph_serde_roundtrip() {
    let mut graph = Graph::new();
    let a_id = graph.add("Node A");
    let b_id = graph.add("Node B");
    graph.connect(a_id, b_id, &Arrow::CHILDREN).unwrap();

    let json = graph.to_json();
    let restored_graph = Graph::from_json(&json).expect("Failed to restore graph from JSON");

    assert_eq!(graph.len(), restored_graph.len());
    assert!(restored_graph.contains(a_id));
    assert!(restored_graph.contains(b_id));
    
    let node_a = restored_graph.get(a_id).unwrap();
    assert_eq!(node_a.data, "Node A".into());
}

#[test]
fn test_invalid_json_graph() {
    let invalid_json = r#"{"invalid": "format"}"#;
    let result = Graph::from_json(invalid_json);
    assert!(result.is_err());
}

#[test]
fn test_bidirectional_invariant_violation() {
    // This test is omitted because Node::linked is pub(crate).
    // Corrupted state can only be injected via JSON for integration tests.
    let corrupted_json = r#"{
        "00000000-0000-0000-0000-000000000001": {
            "data": "A",
            "linked": [{"label": "children", "reverse": "parents", "neighbors": ["00000000-0000-0000-0000-000000000002"]}]
        },
        "00000000-0000-0000-0000-000000000002": {
            "data": "B",
            "linked": []
        }
    }"#;
    let result = Graph::from_json(corrupted_json);
    assert!(result.is_err());
}
