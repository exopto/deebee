use deebee::{Graph, Node, Arrow, Value}; // For nearly all use cases, this is all you will need!

#[test]
fn test_basic() {
    let mut graph = Graph::new();

    // Creating a new node. Data is automatically converted to Value and can be any of the following types which implement Into<Value>:
    // (), bool, i64, u32. i32, isize, f64, f32, &str, String, Vec<Into<Value>>, or HashMap<String, Into<Value>>
    let a = Node::new("data");

    let a_id = graph.insert(a); // Inserts `a` into the graph. Returns a reference.

    let b_id = graph.add(vec!["Magic", "From", "Trait", "Impls"]); // If you're lazy.

    // Arrow offers 5 connection constants by default: `children` and its reverse `parents`, `receiving` and its reverse `pointing`, and `linked`, which goes both ways.
    graph.connect(a_id, b_id, &Arrow::CHILDREN).unwrap(); // All arrows are bidirectional, meaning b knows it is a parent of `a` after connection also.


    if let Some(a) = graph.get_mut(a_id) {
        a.set("daddy")
    }
    
    assert_eq!(Value::Text("daddy".to_string()), graph.get(a_id).unwrap().data);

    assert_eq!(Value::Text("daddy".to_string()), graph.find("daddy").next().unwrap().data);
}