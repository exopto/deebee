use crate::{Graph, Node, Value, Arrow};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[test]
fn test_graph_initialization() {
    let graph = Graph::new();
    assert!(graph.is_empty());
    assert_eq!(graph.len(), 0);
}

#[test]
fn test_node_addition_and_removal() {
    let mut graph = Graph::new();

    let node_id = graph.add("First Node").id;
    assert_eq!(graph.len(), 1);
    assert!(graph.contains(node_id));

    graph.remove(node_id).unwrap();
    assert!(graph.is_empty());
    assert!(!graph.contains(node_id));
}

#[test]
fn test_bidirectional_connections() {
    let mut graph = Graph::new();
    let n1 = graph.add("Node 1").id;
    let n2 = graph.add("Node 2").id;

    // Connect n1 -> n2 via CHILDREN
    graph.connect(n1, n2, &Arrow::CHILDREN).unwrap();

    // Verify n1 points to n2
    assert!(graph.get(n1).unwrap().contains(n2, &Arrow::CHILDREN));
    // Verify n2 reverse-points to n1
    assert!(graph.get(n2).unwrap().contains(n1, &Arrow::PARENTS));

    // Disconnect
    graph.disconnect(n1, n2, &Arrow::CHILDREN).unwrap();
    assert!(!graph.get(n1).unwrap().contains(n2, &Arrow::CHILDREN));
    assert!(!graph.get(n2).unwrap().contains(n1, &Arrow::PARENTS));
}

#[test]
fn test_graph_traversal() {
    let mut graph = Graph::new();
    let n1 = graph.add(1).id;
    let n2 = graph.add(2).id;
    let n3 = graph.add(3).id;
    let n4 = graph.add(4).id;

    // Build tree: n1 -> n2, n1 -> n3, n2 -> n4
    graph.connect(n1, n2, &Arrow::POINTING).unwrap();
    graph.connect(n1, n3, &Arrow::POINTING).unwrap();
    graph.connect(n2, n4, &Arrow::POINTING).unwrap();

    let traversed: HashSet<Uuid> = graph.traverse_all(n1).unwrap().map(|n| n.id).collect();

    assert_eq!(traversed.len(), 4);
    assert!(traversed.contains(&n1));
    assert!(traversed.contains(&n2));
    assert!(traversed.contains(&n3));
    assert!(traversed.contains(&n4));
}

#[test]
fn test_filtered_traversal() {
    let mut graph = Graph::new();
    let n1 = graph.add(1).id;
    let n2 = graph.add(2).id;
    let n3 = graph.add(3).id;

    graph.connect(n1, n2, &Arrow::POINTING).unwrap();
    graph.connect(n1, n3, &Arrow::LINKED).unwrap();

    let pointing_traversed: HashSet<Uuid> = graph.traverse(n1, &Arrow::POINTING).unwrap().map(|n| n.id).collect();
    assert_eq!(pointing_traversed.len(), 2);
    assert!(pointing_traversed.contains(&n1));
    assert!(pointing_traversed.contains(&n2));
    assert!(!pointing_traversed.contains(&n3));
}

#[test]
fn test_find_mechanics() {
    let mut graph = Graph::new();
    let target_val = "target";

    let n1 = graph.add(target_val).id;
    let n2 = graph.add("ignore").id;
    let n3 = graph.add(target_val).id;

    let found: Vec<&Node> = graph.find(target_val).collect();
    assert_eq!(found.len(), 2);
    assert!(found.iter().any(|node| node.id == n1));
    assert!(found.iter().any(|node| node.id == n3));
    assert!(!found.iter().any(|node| node.id == n2));
}

#[test]
fn test_neighbors() {
    let mut graph = Graph::new();
    let n1 = graph.add(1).id;
    let n2 = graph.add(2).id;
    let n3 = graph.add(3).id;

    graph.connect(n1, n2, &Arrow::POINTING).unwrap();
    graph.connect(n1, n3, &Arrow::LINKED).unwrap();

    let neighbors: Vec<&Node> = graph.neighbors(n1, &Arrow::POINTING).unwrap().collect();
    assert_eq!(neighbors.len(), 1);
    assert_eq!(neighbors[0].id, n2);

    let all_neighbors: Vec<(&Arrow, &Node)> = graph.all_neighbors(n1).unwrap().collect();
    assert_eq!(all_neighbors.len(), 2);
}

#[test]
fn test_find_neighbors_and_tree() {
    let mut graph = Graph::new();
    let n1 = graph.add(1).id;
    let n2 = graph.add("target").id;
    let n3 = graph.add("target").id;
    let n4 = graph.add("ignore").id;

    graph.connect(n1, n2, &Arrow::POINTING).unwrap();
    graph.connect(n1, n3, &Arrow::POINTING).unwrap();
    graph.connect(n1, n4, &Arrow::POINTING).unwrap();
    
    // n3 -> n5 (target)
    let n5 = graph.add("target").id;
    graph.connect(n3, n5, &Arrow::POINTING).unwrap();

    let found_neighbors: Vec<&Node> = graph.find_neighbors(n1, "target", &Arrow::POINTING).unwrap().collect();
    assert_eq!(found_neighbors.len(), 2);
    
    let found_tree: Vec<&Node> = graph.find_tree(n1, "target", &Arrow::POINTING).unwrap().collect();
    assert_eq!(found_tree.len(), 3); // n2, n3, n5
}

#[test]
fn test_json_serialization_roundtrip() {
    let mut graph = Graph::new();
    let n1 = graph.add("Alice").id;
    let n2 = graph.add("Bob").id;
    graph.connect(n1, n2, &Arrow::LINKED).unwrap();

    let json_str = graph.to_json();

    let new_graph = Graph::from_json(&json_str).expect("Failed to parse generated JSON");

    assert_eq!(new_graph.len(), 2);
    assert!(new_graph.contains(n1));
    assert!(new_graph.contains(n2));
    assert!(new_graph.get(n1).unwrap().contains(n2, &Arrow::LINKED));
}

// Node tests
#[test]
fn test_arrow_reverse() {
    let custom_arrow = Arrow::new("likes", "liked_by");
    let reversed = custom_arrow.reverse();

    assert_eq!(reversed.label, "liked_by");
    assert_eq!(reversed.reverse_label, "likes");
    assert_eq!(reversed.reverse(), custom_arrow);

    // Test built-in arrows
    assert_eq!(Arrow::CHILDREN.reverse(), Arrow::PARENTS);
    assert_eq!(Arrow::POINTING.reverse(), Arrow::RECEIVING);
}

#[test]
fn test_node_creation() {
    let node = Node::new("test data");
    assert_eq!(node.data, Value::Text("test data".to_string()));
    assert!(node.is_empty());
    assert_eq!(node.len(), 0);
}

#[test]
fn test_node_with_id_and_mutation() {
    let id = Uuid::new_v4();
    let mut node = Node::with_id(42, id);

    assert_eq!(node.id, id);
    assert_eq!(node.data, Value::Int(42));

    node.set(true);
    assert_eq!(node.data, Value::Bool(true));
}

// Value tests
#[test]
fn test_value_from_conversions() {
    assert_eq!(Value::from(()), Value::Null);
    assert_eq!(Value::from(true), Value::Bool(true));
    assert_eq!(Value::from(42), Value::Int(42));
    assert_eq!(Value::from(3.14), Value::Float(3.14));
    assert_eq!(Value::from("hello"), Value::Text("hello".to_string()));

    let vec_val: Value = vec![1, 2, 3].into();
    assert_eq!(vec_val, Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]));
}

#[test]
fn test_serialization_primitives() {
    assert_eq!(Value::Null.serialize(), "null");
    assert_eq!(Value::Bool(true).serialize(), "true");
    assert_eq!(Value::Int(-100).serialize(), "-100");
    assert_eq!(Value::Float(0.5).serialize(), "0.5");
    assert_eq!(Value::Float(f64::NAN).serialize(), "null");
}

#[test]
fn test_escaping_and_unescaping() {
    let original = "Line1\nLine2\t\"Quotes\"\\Slash";
    let escaped = Value::escape(original);
    assert_eq!(escaped, r#""Line1\nLine2\t\"Quotes\"\\Slash""#);

    // Strip the outer quotes for unescape testing as `deserialize` handles the outer quotes
    let unescaped = Value::unescape(&escaped[1..escaped.len()-1]).unwrap();
    assert_eq!(unescaped, original);
}

#[test]
fn test_deserialization_success() {
    assert_eq!(Value::deserialize("null").unwrap(), Value::Null);
    assert_eq!(Value::deserialize("false").unwrap(), Value::Bool(false));
    assert_eq!(Value::deserialize("12345").unwrap(), Value::Int(12345));
    assert_eq!(Value::deserialize("12.34").unwrap(), Value::Float(12.34));
    assert_eq!(Value::deserialize(r#""hello""#).unwrap(), Value::Text("hello".to_string()));
}

#[test]
fn test_roundtrip_complex_structures() {
    let mut map = HashMap::new();
    map.insert("key1".to_string(), Value::Text("value1".to_string()));
    map.insert("key2".to_string(), Value::List(vec![Value::Int(1), Value::Null]));

    let original_val = Value::Map(map);
    let serialized = original_val.serialize();
    let deserialized = Value::deserialize(&serialized).unwrap();

    assert_eq!(original_val, deserialized);
}
