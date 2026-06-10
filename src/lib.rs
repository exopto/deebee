use std::collections::{HashMap, HashSet};
use std::ops::Index;
use uuid::Uuid;

pub mod deejson;
use deejson::Value;

// ADD DEEP AND STUFF

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Arrow {
    label: &'static str,
    reverse_label: &'static str,
}

impl Arrow {
    pub const CHILDREN: Self = Self {
        label: "children",
        reverse_label: "parents",
    };
    pub const PARENTS: Self = Self {
        label: "parents",
        reverse_label: "children",
    };
    pub const POINTING: Self = Self {
        label: "pointing",
        reverse_label: "receiving",
    };
    pub const RECEIVING: Self = Self {
        label: "receiving",
        reverse_label: "pointing",
    };
    pub const LINKED: Self = Self {
        label: "linked",
        reverse_label: "linked",
    };

    pub fn new(label: &'static str, reverse_label: &'static str) -> Self {
        Self {
            label,
            reverse_label,
        }
    }
    pub fn reverse(&self) -> Arrow {
        Self {
            label: self.reverse_label,
            reverse_label: self.label,
        }
    }
}

/// The core of Deebee. Stores a data of type Value as well as nodes it links to (bidirectional).
#[derive(PartialEq, Clone)]
pub struct Node {
    pub id: Uuid,
    pub data: Value,
    linked: HashMap<Arrow, Vec<Uuid>>,
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
    pub fn set(&mut self, data: impl Into<Value>) {
        self.data = data.into()
    }

    /// Gets all nodes of a specific arrow type linked to the node.
    pub fn get(&self, arrow: Arrow) -> &Vec<Uuid> {
        static EMPTY: Vec<Uuid> = vec![];
        self.linked.get(&arrow).unwrap_or(&EMPTY)
    }

    /// Checks if the current node contains the id of another node
    pub fn contains(&self, id: Uuid, arrow: Arrow) -> bool {
        self.linked
            .get(&arrow)
            .is_some_and(|nodes| nodes.contains(&id))
    }

    // pub fn search(&self, func: impl Fn(&Node) -> bool) -> impl Iterator<Item = &Node> {
    //     self.linked.values().filter(move |node| func(node))
    // }
    
    // Search and find need to be over the graph like Graph.search_in() and Graph.find_in() with traverse
    //// search ////
    // find
    // delete
    // Display, Iterator, IntoIterator
}

impl Index<Arrow> for Node {
    type Output = Vec<Uuid>;
    fn index(&self, arrow: Arrow) -> &Self::Output {
        &self.linked[&arrow]
    }
}

////// SUGAR METHODS //////
// add
// link
// point
// receive

// get_children
// get_parents
// get_pointings
// get_incomings

pub struct Graph {
    pub nodes: HashMap<Uuid, Node>,
}

impl Graph {
    /// Creates a new empty graph.
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    /// Inserts a new node to the graph.
    pub fn add(&mut self, data: impl Into<Value>) -> &Node {
        let node = Node::new(data);
        let id = node.id;
        self.nodes.insert(id, node);
        self.nodes
            .get(&id)
            .expect("Node insertion failed for unknown reason.")
    }

    /// Returns an iterator of all nodes where the value matches the Value given.
    pub fn find(&self, data: impl Into<Value>) -> impl Iterator<Item = &Node> {
        let data = data.into();
        self.nodes.values().filter(move |node| node.data == data)
    }

    /// Returns an iterator of all nodes where the value matches the constraints of the given function.
    pub fn search(&self, func: impl Fn(&Node) -> bool) -> impl Iterator<Item = &Node> {
        self.nodes.values().filter(move |node| func(node))
    }

    /// Gets node by UUID.
    pub fn get(&self, id: Uuid) -> Option<&Node> {
        self.nodes.get(&id)
    }

    /// Gets mutable node by UUID.
    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }

    /// Clears entire graph.
    pub fn clear(&mut self) {
        self.nodes.clear()
    }

    /// Extends graph from the given node map.
    pub fn extend(&mut self, extension: impl IntoIterator<Item = (Uuid, Node)>) {
        self.nodes.extend(extension)
    }

    /// Checks if ID exists in graph.
    pub fn contains(&self, id: Uuid) -> bool {
        self.nodes.contains_key(&id)
    }

    /// Returns map of all nodes in the graph.
    pub fn to_map(&self) -> &HashMap<Uuid, Node> {
        &self.nodes
    }
    
    /// Returns JSON-like String of all nodes in the graph. 
    pub fn to_json(&self) -> String {
        let entries: Vec<String> = self
            .nodes
            .iter()
            .map(|(key, node)| format!("  \"{key}\": {}", &node.data.serialize()))
            .collect();

        format!("{{\n{}\n}}", entries.join(",\n"))
    }

    /// Traverses through the graph starting from a given node, through the arrow type specified. Uses Depth-First Search.
    pub fn traverse(&self, starting_node: Uuid, arrow: Arrow) -> impl Iterator<Item = Uuid> {
        let mut stack = vec![starting_node]; // Nodes to visit and track descendants
        let mut seen: HashSet<Uuid> = HashSet::from([starting_node]);

        std::iter::from_fn(move || {
            if let Some(node) = stack.pop() {
                for neighbor in &self.nodes[&node].linked[&arrow] {
                    if !seen.contains(neighbor) {
                        seen.insert(*neighbor);
                        stack.push(*neighbor);
                    }
                }
                Some(node)
            } else {
                None
            }
        })
    }

    /// Connects one node on the graph to another using the given arrow type.
    pub fn connect(&mut self, from_id: Uuid, to_id: Uuid, arrow: Arrow) -> Result<(), &'static str> {
        if !self.nodes.contains_key(&from_id) || !self.nodes.contains_key(&to_id) {
            Err("Node given not in graph.")
        } else {
            if !self.nodes[&from_id].linked[&arrow].contains(&to_id) {
                self.nodes
                    .get_mut(&from_id)
                    .unwrap()
                    .linked
                    .entry(arrow)
                    .or_default()
                    .push(to_id);
                self.nodes
                    .get_mut(&to_id)
                    .unwrap()
                    .linked
                    .entry(arrow.reverse())
                    .or_default()
                    .push(from_id);
            }
            Ok(())
        }
    }

    // to_json
    // from_json
    // Display, Iterator, IntoIterator
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl From<HashMap<Uuid, Node>> for Graph {
    fn from(data: HashMap<Uuid, Node>) -> Self {
        Graph { nodes: data }
    }
}