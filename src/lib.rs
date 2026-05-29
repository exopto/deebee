use std::collections::HashMap;
use uuid::Uuid;

#[derive(PartialEq, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    Bytes(Vec<u8>),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
    Custom(Box<dyn CustomValue>),
}

pub trait CustomValue {
    fn eq_box(&self, other: &dyn CustomValue) -> bool;
    fn clone_box(&self) -> Box<dyn CustomValue>;
    fn as_any(&self) -> &dyn std::any::Any;
    fn to_value(&self) -> Value {Value::Null}
}

impl PartialEq for dyn CustomValue {fn eq(&self, other: &Self) -> bool {self.eq_box(other)}}
impl Clone for Box<dyn CustomValue> {fn clone(&self) -> Self {self.clone_box()}}

impl<T: PartialEq + Clone + 'static> CustomValue for T {
    fn eq_box(&self, other: &dyn CustomValue) -> bool {other.as_any().downcast_ref::<T>() == Some(self)}
    fn clone_box(&self) -> Box<dyn CustomValue> {Box::new(self.clone())}
    fn as_any(&self) -> &dyn std::any::Any {self}
}

#[derive(Clone, Copy, Debug)]
struct Arrow {
    label: &'static str,
    reverse_label: &'static str
}

impl Arrow {
    pub const CHILD: Self = Self {label: "children", reverse_label: "parents"};
    pub const PARENT: Self = Self {label: "parents", reverse_label: "children"};
    pub const POINTING: Self = Self {label: "pointing", reverse_label: "receiving"};
    pub const RECEIVING: Self = Self {label: "receiving", reverse_label: "pointing"};

    pub fn new(label: &'static str, reverse_label: &'static str) -> Self {Self {label, reverse_label}}
    pub fn reverse(&self) -> Arrow {Self {label: self.reverse_label, reverse_label: self.label}}
}




pub struct Node {
    pub id: Uuid,
    pub data: Value,
    out_nodes: HashMap<String, Vec<Uuid>>,
    in_nodes:  HashMap<String, Vec<Uuid>>,
}

impl Node {
    pub fn new(data: impl Into<Value>) -> Self {
        Self::with_id(data, Uuid::new_v4())
    }

    pub fn with_id(data: impl Into<Value>, id: Uuid) -> Self {
        Node {
            id,
            data: data.into(),
            out_nodes: HashMap::new(),
            in_nodes:  HashMap::new(),
        }
    }

    fn traverse(&self, _arrow: Arrow) -> impl Iterator<Item = Uuid> {
        std::iter::once(self.id)
    }

    // add
    // link
    // point
    // receive
    // get
    // get_parents
    // get_pointings
    // get_incomings
    // find
    // set
    // connect
    // search
    // get_boxes
    // delete
    // contains
    // Display, Debug, Index, IndexMut, Iterator, IntoIterator, PartialEq, Clone
}






pub struct Graph {
    pub nodes: HashMap<Uuid, Node>,
}

impl Graph {
    /// Creates a new graph
    pub fn new() -> Self {
        Graph { nodes: HashMap::new() }
    }

    /// Inserts a new node to the graph.
    pub fn add(&mut self, data: impl Into<Value>) -> &Node {
        let node = Node::new(data);
        let id = node.id;
        self.nodes.insert(id, node);
        self.nodes.get(&id).expect("Node insertion failed for unknown reason.")
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
    pub fn get(&self, id: Uuid) -> Option<&Node> {self.nodes.get(&id)}

    /// Gets mutable node by UUID.
    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut Node> {self.nodes.get_mut(&id)}

    /// Clears entire graph.
    pub fn clear(&mut self) {self.nodes.clear()}

    /// Returns map of all nodes in the graph.
    pub fn to_map(&self) -> &HashMap<Uuid, Node> {&self.nodes}

    /// Extends graph from the given node map.
    pub fn from_map(&mut self, extension: impl IntoIterator<Item = (Uuid, Node)>) {self.nodes.extend(extension)}

    // delete
    // to_json
    // from_json
}