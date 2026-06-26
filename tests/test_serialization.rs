use deebee::{Graph, Arrow};
use std::fs;

#[test]
fn test_serialization_overall() {
    // Build a small family graph
    let mut graph = Graph::new();
    
    let grandpa = graph.add("Grandpa Joe").id;
    let dad = graph.add("Dad").id;
    let bob = graph.add("Bob").id;
    
    graph.connect(grandpa, dad, &Arrow::CHILDREN).unwrap();
    graph.connect(dad, bob, &Arrow::CHILDREN).unwrap();
    
    // Serialize and save
    let json = graph.to_json();
    fs::write("family.json", &json).expect("Failed to write file.");
    println!("Saved:\n{json}");
    
    // Load and deserialize
    // let loaded = fs::read_to_string("family.json").expect("Failed to read file.");
    // let graph: Graph = loaded.parse().expect("Failed to deserialize.");
    
    // // Verify
    // let bob_node = graph.find("Bob").next().expect("Bob not found.");
    // let dad_node = graph.find("Dad").next().expect("Dad not found.");
    
    // println!("\nBob's parents: {:?}", bob_node.get(&Arrow::PARENTS));
    // println!("Dad's children: {:?}", dad_node.get(&Arrow::CHILDREN));

    // assert_eq!(bob_node.get(&Arrow::PARENTS), Some(&HashSet::from([dad])));
    // assert_eq!(dad_node.get(&Arrow::CHILDREN), Some(&HashSet::from([bob])));
}