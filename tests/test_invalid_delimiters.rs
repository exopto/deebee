use deebee::Value;

#[test]
fn test_unbalanced_delimiters() {
    let invalid = vec![
        "}{",
        "][",
        "{]",
        "[}",
        "[1, 2, }]",
        "{\"key\": ]\"value\"}",
        "[[1, 2], }]",
        "{\"a\": [1, 2}]",
        "{}[{]}",
        "{}}}{{{}",
        "\"incomplete escape \\",
        "\"invalid unicode \\u123\"",
        // NOTE: RIGHT NOW THE DESERIALIZER ACCEPTS THE FOLLOWING INVALID JSON, V0.4.0 WILL HOPEFULLY FIX THIS
        // "{\"key\": \"value\",}", // Trailing comma
        // "{ \"key\": \"value\" , }", // Trailing comma
        // "[1, 2, 3,]", // Trailing comma
        // "{\"key\": }", // Missing value
        // "{ : \"value\"}", // Missing key
        // "{\"key\": \"val\" \"key2\": \"val2\"}", // Missing comma
    ];
    for input in &invalid {
        assert!(Value::deserialize(input).is_err(), "Expected Err for: {input}");
    }
}