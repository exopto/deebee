# Deebee
A super tiny graph database built on a philosophy of simplicity, flexibility, and sugar. It is written in Rust for speed, stability, and idiomatic design, although the API is streamlined greatly for use in other languages, such as Python. Deebee does not have a query language by design, rather the progamming language itself filling that gap with each method made to read like standard English. It is written in X lines of easy-to-understand code (for "clean design", AKA so that I don't pass out with proc macros before I even finish the Book).

Deebee consists of only four primitives: the Graph, storing nodes, the Node, storing values and connecting to other nodes, the Arrow, the connection to the other nodes, and the Value, dynamic data stored by the Node.

[Repo](https://github.com/pythonkid90/deebee) | [Crate](https://crates.io/crates/deebee) | [Docs](https://docs.rs/deebee) | [Blog](https://dev.to/rusty_pythonista)
## Installation


## Examples
```rust

```

## Roadmap
- Clean API code further for idiomaticity and remove usage of recursion
- Create a Python wrapper using PyO3 and publish to PyPI