use deebee::{Graph, Arrow};
use std::fs;
use std::collections::{HashSet};

#[test]
fn test_serialization_overall() {
    // Build a small family graph
    let mut graph = Graph::new();
    
    let grandpa = graph.add("Grandpa Joe");
    let dad = graph.add("Dad");
    let bob = graph.add("Bob");
    
    graph.connect(grandpa, dad, &Arrow::CHILDREN).unwrap();
    graph.connect(dad, bob, &Arrow::CHILDREN).unwrap();
    
    // Serialize and save
    let json = graph.to_json();
    fs::write(".local/family.json", &json).expect("Failed to write file.");
    println!("Saved:\n{json}");
    
    // Load and deserialize
    let loaded = fs::read_to_string(".local/family.json").expect("Failed to read file.");
    let graph: Graph = loaded.parse().expect("Failed to deserialize.");
    
    // Verify
    let bob_node = graph.find("Bob").next().expect("Bob not found.");
    let dad_node = graph.find("Dad").next().expect("Dad not found.");
    
    println!("\nBob's parents: {:?}", bob_node.get(&Arrow::PARENTS));
    println!("Dad's children: {:?}", dad_node.get(&Arrow::CHILDREN));

    assert_eq!(bob_node.get(&Arrow::PARENTS), Some(&HashSet::from([dad])));
    assert_eq!(dad_node.get(&Arrow::CHILDREN), Some(&HashSet::from([bob])));
}

#[test]
fn test_json_serialization_roundtrip() {
    let mut graph = Graph::new();
    let n1 = graph.add("Alice");
    let n2 = graph.add("Bob");
    graph.connect(n1, n2, &Arrow::LINKED).unwrap();

    let json_str = graph.to_json();

    let new_graph = Graph::from_json(&json_str).expect("Failed to parse generated JSON");

    assert_eq!(new_graph.len(), 2);
    assert!(new_graph.contains(n1));
    assert!(new_graph.contains(n2));
    assert!(new_graph.get(n1).unwrap().contains(n2, &Arrow::LINKED));
}