use deebee::deejson::{Value};
use deebee::{Arrow, Graph, Node};


use std::collections::HashMap;
use uuid::Uuid;

// Assuming Graph, Node, and Value are in scope

fn main() {
    // 1. Test an empty graph
    let empty_graph = Graph::new();
    println!("--- Empty Graph ---");
    println!("{}\n", empty_graph.to_json_string());
    assert_eq!(empty_graph.to_json_string(), "{\n\n}");

    // 2. Setup deterministic UUIDs for predictable testing
    let id_null_bool = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let id_numbers   = Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap();
    let id_text      = Uuid::parse_str("00000000-0000-0000-0000-000000000003").unwrap();
    let id_complex   = Uuid::parse_str("00000000-0000-0000-0000-000000000004").unwrap();

    let mut graph = Graph::new();

    // Edge Case 1: Null and Booleans
    graph.nodes.insert(id_null_bool, Node::with_id(Value::Null, id_null_bool));
    // Overriding it just to test bools too
    graph.nodes.insert(id_null_bool, Node::with_id(true, id_null_bool));

    // Edge Case 2: Numbers, including NaN / Infinity handling
    let nan_val = std::f64::NAN;
    let inf_val = std::f64::INFINITY;
    graph.nodes.insert(id_numbers, Node::with_id(Value::List(vec![
        Value::from(42), 
        Value::from(-999), 
        Value::from(3.1415), 
        Value::from(nan_val), 
        Value::from(inf_val)
    ]), id_numbers));

    // Edge Case 3: Tricky Text (Escapes, Newlines, Control Chars)
    let tricky_string = "Line1\nLine2\t\"Quote\"\\Backslash\x08Backspace\x1FControl";
    graph.nodes.insert(id_text, Node::with_id(tricky_string, id_text));

    // Edge Case 4: Complex Nested Map and Bytes
    let mut map = HashMap::new();
    map.insert("nested_list".to_string(), Value::List(vec![Value::from(1), Value::from(2)]));
    map.insert("raw_bytes".to_string(), Value::Bytes(vec![0xDE, 0xAD, 0xBE, 0xEF]));
    
    graph.nodes.insert(id_complex, Node::with_id(Value::Map(map), id_complex));

    // 3. Print out the populated graph
    println!("--- Populated Graph ---");
    let json_output = graph.to_json_string();
    println!("{}", json_output);

    // Note: Because HashMap ordering is non-deterministic in Rust, 
    // the order of the keys inside `to_json_string` (and inside our Map edge case) 
    // will shuffle every time you run it. 
    // Manual visual inspection of the output string is best here!
}