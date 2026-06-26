use deebee::Value;

#[test]
fn test_deserialization_chaos() {
    let chaos_inputs = vec![
        "".to_string(),
        " ".to_string(),
        "\n".to_string(),
        "{".to_string(),
        "}".to_string(),
        "[".to_string(),
        "]".to_string(),
        "\"".to_string(),
        "\\".to_string(),
        "null".to_string(),
        "true".to_string(),
        "false".to_string(),
        "123".to_string(),
        "-123".to_string(),
        "123.456".to_string(),
        "-123.456".to_string(),
        "\"\"".to_string(),
        "\"\\n\"".to_string(),
        "\"\\r\"".to_string(),
        "\"\\t\"".to_string(),
        "\"\\\\\"".to_string(),
        "\"\\\"\"".to_string(),
        "\"\\u0000\"".to_string(),
        "\"\\uFFFF\"".to_string(),
        "\"\\uZZZZ\"".to_string(),
        "{\"key\": \"value\"}".to_string(),
        "{\"key\": 123}".to_string(),
        "{\"key\": [1, 2, 3]}".to_string(),
        "{\"key\": {\"inner\": \"val\"}}".to_string(),
        "[1, 2, 3]".to_string(),
        "[\"a\", \"b\"]".to_string(),
        "[{ \"a\": 1 }, { \"b\": 2 }]".to_string(),
        "{\"key\": \"value\",}".to_string(), // Trailing comma
        "{ \"key\": \"value\" , }".to_string(), // Trailing comma
        "[1, 2, 3,]".to_string(), // Trailing comma
        "{\"key\": }".to_string(), // Missing value
        "{ : \"value\"}".to_string(), // Missing key
        "{\"key\": \"val\" \"key2\": \"val2\"}".to_string(), // Missing comma
        "\"incomplete escape \\".to_string(),
        "\"invalid unicode \\u123\"".to_string(),
        "[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[".to_string(), // Deep nesting
        "]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]".to_string(), // Deep nesting
        "{\"a\":{\"a\":{\"a\":{\"a\":{\"a\":{\"a\":{\"a\":{\"a\":{\"a\":{\"a\":{}}}}}}}}}}}".to_string(), // Deep map nesting
        format!("\"{}\"", "a".repeat(10000)), // Very long string
        format!("[{}{}]", "1,".repeat(10000), "1"), // Very long list
    ];

    for input in chaos_inputs {
        // We don't care if it returns Err, we only care that it DOES NOT PANIC.
        let _ = Value::deserialize(&input);
    }
}

#[test]
fn test_random_garbage() {
    use std::time::{SystemTime, UNIX_EPOCH};

    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut rng = SimpleRng::new(seed);

    for _ in 0..10000 {
        let mut input = String::new();
        let len = rng.gen_range(0..100);
        for _ in 0..len {
            let char = rng.gen_char();
            input.push(char);
        }
        let _ = Value::deserialize(&input);
    }
}

struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }

    fn gen_range(&mut self, range: std::ops::Range<usize>) -> usize {
        (self.next() as usize % (range.end - range.start)) + range.start
    }

    fn gen_char(&mut self) -> char {
        let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 \t\n\r,.:;[]{}()\"'\\-/_";
        let idx = self.gen_range(0..chars.len());
        chars.chars().nth(idx).unwrap()
    }
}
