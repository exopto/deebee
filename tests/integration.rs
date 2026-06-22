use std::collections::HashMap;
use deebee::{Graph, Value, Arrow, Node}; 

#[test]
fn test_social_network_scenario() {
    let mut db = Graph::new();

    // 1. Create a custom arrow for friendships
    let follows = Arrow::new("follows", "followed_by");

    // 2. Add users using HashMaps converted to Value::Map
    let mut alice_data = HashMap::new();
    alice_data.insert("name".to_string(), Value::from("Alice"));
    alice_data.insert("age".to_string(), Value::from(28));
    
    let mut bob_data = HashMap::new();
    bob_data.insert("name".to_string(), Value::from("Bob"));
    bob_data.insert("age".to_string(), Value::from(32));

    let alice_id = db.add(alice_data).id;
    let bob_id = db.add(bob_data).id;
    let charlie_id = db.add("Charlie (Minimal Profile)").id;

    // 3. Form connections
    db.connect(alice_id, bob_id, follows).unwrap();
    db.connect(bob_id, charlie_id, follows).unwrap();

    // 4. Verify invariants through the public API
    assert_eq!(db.len(), 3);
    
    let alice_node = db.get(alice_id).unwrap();
    assert!(alice_node.contains(bob_id, follows));
    
    // Check that Bob was "followed_by" Alice implicitly
    let bob_node = db.get(bob_id).unwrap();
    assert!(bob_node.contains(alice_id, follows.reverse()));

    // 5. Querying logic
    let charlie_tree: Vec<_> = db.find_tree(alice_id, "Charlie (Minimal Profile)").unwrap().collect();
    // find_tree iterates through all descendants of Alice to find the target Value.
    assert_eq!(charlie_tree.len(), 1);
    assert_eq!(charlie_tree[0], charlie_id);

    // 6. Test Data integrity / Removal
    db.remove(bob_id).unwrap();
    
    // Removing Bob should sever Alice's connection to him
    let alice_node_after = db.get(alice_id).unwrap();
    assert!(!alice_node_after.contains(bob_id, follows));
    assert_eq!(db.len(), 2);
}

#[test]
fn test_from_map_and_manual_insertion() {
    let mut manual_map = std::collections::HashMap::new();
    
    let node1 = Node::new(100);
    let node2 = Node::new(200);
    
    manual_map.insert(node1.id, node1.clone());
    manual_map.insert(node2.id, node2.clone());

    // Fails bidirectionality test because they aren't properly linked, 
    // but works fine for disconnected nodes.
    let mut graph = Graph::from_map(manual_map).unwrap();
    
    assert_eq!(graph.len(), 2);
    
    // Test the insert command (inserting an owned node)
    let node3 = Node::new(300);
    graph.insert(node3.clone());
    
    assert!(graph.contains(node3.id));
}