#![doc = include_str!("../README.md")]

pub mod value;
pub use value::Value;

pub mod node;
pub use node::{Node, Arrow};

pub mod graph;
pub use graph::Graph;

#[derive(Debug, PartialEq, Clone)]
/// Deebee's error enum. Serves as the `Err` variant for whenever any function or method in the library returns a result. See documentation for the variants below.
pub enum DeebeeError {
    #[doc = include_str!("../docs/lib/nodenotfound.md")] NodeNotFound(uuid::Uuid),
    #[doc = include_str!("../docs/lib/invalidjson.md")] DeserializeError(String),
    #[doc = include_str!("../docs/lib/invalidcreation.md")] InvalidGraphFormat(String),
}

impl std::fmt::Display for DeebeeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NodeNotFound(id) => write!(f, "Node ID {} not in graph.", id),
            Self::DeserializeError(msg) => write!(f, "Graph cannot be deserialized. Deserializer noted \"{}\"", msg),
            Self::InvalidGraphFormat(msg) => write!(f, "Graph format is invalid: {}", msg),
        }
    }
}

impl std::error::Error for DeebeeError {}

impl From<uuid::Error> for DeebeeError {fn from(err: uuid::Error) -> Self {Self::InvalidGraphFormat(err.to_string().into())}}