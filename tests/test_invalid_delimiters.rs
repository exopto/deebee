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
        "{}[{]}"
    ];
    for input in &invalid {
        assert!(Value::deserialize(input).is_err(), "Expected Err for: {input}");
    }
}