#![doc = include_str!("../README.md")]

pub mod value;
pub use value::Value;

pub mod node;
pub use node::{Node, Arrow};

pub mod graph;
pub use graph::Graph;