# Deebee TODO List

## Feature Gaps & Improvements
- [ ] **Limited Querying**: Add `find_where` to allow querying nodes with a predicate closure instead of only exact `Value` matches.
- [ ] **Non-Deterministic JSON Serialization**: Sort keys in `Value::Map` during serialization to ensure consistent output.
- [ ] **Limited Traversal Algorithms**: Implement Breadth-First Search (BFS) in addition to the current DFS in `Graph::traverse`.
- [ ] **Idiomatic Error Handling**: Replace `&'static str` and `String` errors with a custom `DeebeeError` enum.
- [ ] **Robust JSON Parsing**: Increase test coverage for `Value::parse_collection` to handle edge cases in malformed JSON.
