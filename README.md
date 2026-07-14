# Deebee: Modeling Relationships with Swag
Deebee is a lightweight graph database engine written in Rust, with a philosophy of simplicity, flexibility, and sugar. While it makes use of idiomatic Rust patterns, it sticks to the bare minimum so that you can focus on doing what you need to do. It inverts the traditional model of edges being the primary structure in a graph, each edge sort of owning or containing the nodes; rather, the graph owns the nodes directly, making it easier to reason about the graph, especially from those coming from tabular databases.

By design, the library does not use or support a query language; rather, it is completely Rust-native and each method is made to read like standard English. It is written in 625 lines of verbose but relatively high-level Rust (including whitespace but excluding tests, only 514 without whitespace). In addition, its only required dependency is the `uuid` crate, with `serde` as a feature.

Deebee consists of only four primitives: the Graph, storing nodes, the Node, storing values and connecting to other nodes, the Arrow, the connection to the other nodes, and the Value, dynamic data stored by the Node. Each primitive only knows about the other primitives lower on the hierarchy than it; e.g., the Graph knows about the Node, Arrow, and Value, being at the very top, while the Arrow knows only about the Node and Value, being on the same level as the Node and above the Value.

It should not panic unless something is really really wrong, and if it somehow does please report an issue to the GitHub repo, regardless of if you've ever used GitHub before. 

You are reading the documentation and README for Deebee 2026.6.11, an in-progress release (see the [release schedule](#release-schedule)). For the most recent released version (v0.3.1) available on Cargo, please refer to [here](https://github.com/exopto/deebee/tree/v0.3.1).

[Repo](https://github.com/exopto/deebee) | [Crate](https://crates.io/crates/deebee) | [Docs](https://docs.rs/deebee) | [Book (unreleased)](https://exopto.github.io/deebee)

## Installation
For now, Deebee only exists as a Rust crate (while v0.1.0 was written in Python, it has many known design issues and code smells), so the easiest way to get Deebee is to run `cargo add deebee` in your shell while inside your crate root, assuming you have Cargo and Rust installed of course. Troubleshooting on how to install Rust is left as an exercise to the reader.

```shell
cargo add deebee
```

## Examples
This is a crude example to set the stage, and does not showcase anywhere near all of Deebee's features. More comprehensive examples and documentation can be seen in the aforementioned [Deebee Book](https://exopto.github.io/deebee).

```rust
use deebee::{Graph, Node, Arrow}; // For nearly all use cases, this is all you will need!

fn main() {
    let mut graph = Graph::new();

    // Creating a new node. Data is automatically converted to Value and can be any of the following types which implement Into<Value>:
    // (), bool, i64, u32. i32, isize, f64, f32, &str, String, Vec<Into<Value>>, or HashMap<String, Into<Value>>
    let a = Node::new("data");

    let a_id = graph.insert(a); // Inserts `a` into the graph. Returns a UUID that can be used to retrieve the node.

    let b_id = graph.add(vec!["Magic", "From", "Trait", "Impls"]); // If you're lazy.

    // Arrow offers 5 connection constants by default: `children` and its reverse `parents`, `receiving` and its reverse `pointing`, and `linked`, which goes both ways.
    graph.connect(a_id, b_id, &Arrow::CHILDREN); // All arrows are bidirectional, meaning b knows it is a parent of `a` after connection also.
}
```

## Roadmap
Version numbers are an estimate and subject to change.

- Increase robustness of hand-rolled parser, possibly rewriting from scratch (v0.4.0)
- Write Deebee book to go more in detail on how (v0.4.0)
- Write docs for the internal modules and overhaul the function-level doc comments to include more examples and descriptions of behavior. (v0.4.0)
- Continue simplifying and optimizing API, reducing boilerplate while taking note of how the library will be actually used (v0.4.0+)
- Create a Python wrapper using PyO3 and publish to PyPI (v0.4.0+)
- Create a JS wrapper using `wasm-pack` and publish to NPM (v0.5.0+)

## Release Schedule
All commits are added on the `dev` branch, whether they compile or not, such that this branch reflects the current state of the project but often cannot be used in any way. Note that if you are reading this text on that branch, the versioning information above may be outdated or otherwise incorrect.

Commits that pass the test suite, compile, and are not missing any crucial features will also be published to the `main` branch as a calendar-dated rolling release, though some aspects of the library (including the README and documentation) may not be fully fleshed out.

Commits where the API is stabilized and all planned changes for the cycle are done will also correspond to a git tag and version on crates.io.

## Disclosure
While AI was used in the development of Deebee for work such as code formatting, bugfixing, and testing, all final design decisions were made by a real human and any sections of the code generated by AI have been thoroughly reviewed and understood (except for tests, which currently only act as a baseline to confirm the library at least is functional). This document itself is completely human-generated (unless you count spellcheck). Please clap.