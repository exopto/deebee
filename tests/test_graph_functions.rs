use deebee::{Graph, Node, Arrow};
use std::collections::{HashSet};
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

    let node_id = graph.add("First Node");
    assert_eq!(graph.len(), 1);
    assert!(graph.contains(node_id));

    graph.remove(node_id).unwrap();
    assert!(graph.is_empty());
    assert!(!graph.contains(node_id));
}

#[test]
fn test_bidirectional_connections() {
    let mut graph = Graph::new();
    let n1 = graph.add("Node 1");
    let n2 = graph.add("Node 2");

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
    let n1 = graph.add(1);
    let n2 = graph.add(2);
    let n3 = graph.add(3);
    let n4 = graph.add(4);

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
    let n1 = graph.add(1);
    let n2 = graph.add(2);
    let n3 = graph.add(3);

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

    let n1 = graph.add(target_val);
    let n2 = graph.add("ignore");
    let n3 = graph.add(target_val);

    let found: Vec<&Node> = graph.find(target_val).collect();
    assert_eq!(found.len(), 2);
    assert!(found.iter().any(|node| node.id == n1));
    assert!(found.iter().any(|node| node.id == n3));
    assert!(!found.iter().any(|node| node.id == n2));
}

#[test]
fn test_neighbors() {
    let mut graph = Graph::new();
    let n1 = graph.add(1);
    let n2 = graph.add(2);
    let n3 = graph.add(3);

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
    let n1 = graph.add(1);
    let n2 = graph.add("target");
    let n3 = graph.add("target");
    let n4 = graph.add("ignore");

    graph.connect(n1, n2, &Arrow::POINTING).unwrap();
    graph.connect(n1, n3, &Arrow::POINTING).unwrap();
    graph.connect(n1, n4, &Arrow::POINTING).unwrap();
    
    // n3 -> n5 (target)
    let n5 = graph.add("target");
    graph.connect(n3, n5, &Arrow::POINTING).unwrap();

    let found_neighbors: Vec<&Node> = graph.find_neighbors(n1, "target", &Arrow::POINTING).unwrap().collect();
    assert_eq!(found_neighbors.len(), 2);
    
    let found_tree: Vec<&Node> = graph.find_tree(n1, "target", &Arrow::POINTING).unwrap().collect();
    assert_eq!(found_tree.len(), 3); // n2, n3, n5
}