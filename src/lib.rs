#![doc = include_str!("../README.md")]

pub mod value;
pub use value::Value;

pub mod node;
pub use node::{Node, Arrow};

pub mod graph;
pub use graph::Graph;

#[derive(Debug, PartialEq, Clone)]
pub enum DeebeeError {
    NodeNotFound(uuid::Uuid), InvalidJson(String), InvalidCreation(String),
}

impl std::fmt::Display for DeebeeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NodeNotFound(id) => write!(f, "Node ID {} not in graph.", id),
            Self::InvalidJson(msg) => write!(f, "JSON Deserialization failed: {}", msg),
            Self::InvalidCreation(msg) => write!(f, "Invalid creation: {}", msg),
        }
    }
}

impl std::error::Error for DeebeeError {}

impl From<uuid::Error> for DeebeeError {fn from(err: uuid::Error) -> Self {Self::InvalidCreation(err.to_string().into())}}

#[cfg(test)] mod tests;
