use std::collections::{HashMap, HashSet};
use std::ops::Index;
use uuid::Uuid;
use std::fmt;

use crate::Value;

/// A thin wrapper type, connecting nodes bidirectionally with a label and acting as the key to access node neighbors.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Arrow {
    label: &'static str,
    reverse_label: &'static str,
}

impl Arrow {
    pub const CHILDREN: Self = Self {label: "children", reverse_label: "parents",};
    pub const PARENTS: Self = Self {label: "parents", reverse_label: "children"};
    pub const POINTING: Self = Self {label: "pointing", reverse_label: "receiving"};
    pub const RECEIVING: Self = Self {label: "receiving", reverse_label: "pointing"};
    pub const LINKED: Self = Self {label: "linked", reverse_label: "linked"};

    /// Creates a new arrow with a label and the inverse of that label (the label of the arrow going the other direction). 
    pub fn new(label: &'static str, reverse_label: &'static str) -> Self {Self {label, reverse_label}}

    /// Returns the reversed version of the arrow called on; i.e., the arrow that the node's neighbor would have if the arrow points to it.
    pub fn reverse(&self) -> Arrow {Self {label: self.reverse_label, reverse_label: self.label}}
}

/// The core of Deebee. Stores a data of type Value as well as nodes it links to (bidirectional).
#[derive(PartialEq, Clone, Debug)]
pub struct Node {
    pub id: Uuid,
    pub data: Value,
    pub(crate) linked: HashMap<Arrow, HashSet<Uuid>>,
}

impl Node {
    /// Creates a new node with arbitrary data that will automatically be converted to Value.
    pub fn new(data: impl Into<Value>) -> Self {
        Self::with_id(data, Uuid::new_v4())
    }

    /// Creates a new node with arbitrary data that will automatically be converted to Value and a custom UUID.
    pub fn with_id(data: impl Into<Value>, id: Uuid) -> Self {
        Node {
            id,
            data: data.into(),
            linked: HashMap::new(),
        }
    }

    /// Sets arbitrary data of the node that will automatically be converted to Value.
    pub fn set(&mut self, data: impl Into<Value>) {self.data = data.into()}

    /// Gets all nodes of a specific arrow type linked to the node.
    pub fn get(&self, arrow: Arrow) -> Option<&HashSet<Uuid>> {self.linked.get(&arrow)}

    /// Checks if the current node contains the ID of another node.
    pub fn contains(&self, id: Uuid, arrow: Arrow) -> bool {
        self.linked.get(&arrow).is_some_and(|nodes| nodes.contains(&id))
    }

    /// Iterates through all the arrow types of the linked nodes, with a reference to all node IDs of each arrow. Does not expose the Nodes themselves.
    pub fn arrows(&self) -> impl Iterator<Item = (&Arrow, &HashSet<Uuid>)> {self.linked.iter()}

    /// Iterates through all the node IDs, returning a tuple with the arrow to the node and the ID of the node.
    pub fn iter(&self) -> impl Iterator<Item = (&Arrow, &Uuid)> {
        self.linked.iter().flat_map(|(arrow, id_list)| id_list.iter().map(move |id| (arrow, id)))
    }

    /// Returns the number of linked elements to the node.
    pub fn len(&self) -> usize {self.linked.values().map(|ids| ids.len()).sum()}

    /// Check if there are any linked nodes.
    pub fn is_empty(&self) -> bool {self.linked.is_empty()}
}

impl Index<Arrow> for Node {
    type Output = HashSet<Uuid>;
    fn index(&self, arrow: Arrow) -> &Self::Output {
        &self.linked[&arrow]
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.data)
    }
}

impl<'a> IntoIterator for &'a Node {
    type Item = (&'a Arrow, &'a Uuid);
    type IntoIter = Box<dyn Iterator<Item = (&'a Arrow, &'a Uuid)> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.linked.iter().flat_map(|(arrow, id_list)| id_list.iter().map(move |id| (arrow, id))))
    }
}