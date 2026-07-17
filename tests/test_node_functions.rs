use deebee::{Node, Value, Arrow};
use uuid::Uuid;

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
