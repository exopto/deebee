use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use crate::{Node, Value, Arrow};

pub struct Graph {
    pub nodes: HashMap<Uuid, Node>,
    pub arrows: HashSet<Arrow>
}

impl Graph {
    /// Creates a new empty graph.
    pub fn new() -> Self {Graph {nodes: HashMap::new(), arrows: HashSet::new()}}

    /// Gets node by UUID.
    pub fn get(&self, id: Uuid) -> Option<&Node> {self.nodes.get(&id)}

    /// Gets mutable node by UUID.
    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut Node> {self.nodes.get_mut(&id)}

    /// Clears entire graph.
    pub fn clear(&mut self) {self.nodes.clear()}

    /// Extends graph from the given node map.
    pub fn extend(&mut self, extension: impl IntoIterator<Item = (Uuid, Node)>) {self.nodes.extend(extension)}

    /// Checks if ID exists in graph.
    pub fn contains(&self, id: Uuid) -> bool {self.nodes.contains_key(&id)}

    /// Returns an iterator through all the nodes of the graph.
    pub fn iter(&self) -> impl Iterator<Item = &Node> {self.nodes.values()}

    /// Returns map of all nodes in the graph.
    pub fn to_map(&self) -> &HashMap<Uuid, Node> {&self.nodes}
    
    /// Returns JSON-like String of all nodes in the graph. 
    pub fn to_json(&self) -> String {
        let entries: Vec<String> = self
            .nodes
            .iter()
            .map(|(key, node)| format!("  \"{key}\": {{\"data\": {}, \"linked\": {:?}}}", &node.data.serialize(), node.linked))
            .collect();

        format!("{{\n{}\n}}", entries.join(",\n"))
    }

    /// Inserts a new node to the graph.
    pub fn add(&mut self, data: impl Into<Value>) -> &Node {
        let node = Node::new(data);
        let id = node.id;
        self.nodes.insert(id, node);
        self.nodes.get(&id).expect("Node insertion failed for unknown reason.")
    }

    pub fn insert(&mut self, id: Uuid) 

    /// Returns an iterator of all nodes where the value matches the Value given.
    pub fn find(&self, data: impl Into<Value>) -> impl Iterator<Item = &Node> {
        let data = data.into();
        self.nodes.values().filter(move |node| node.data == data)
    }

    /// Connects one node on the graph to another using the given arrow type.
    pub fn connect(&mut self, from_id: Uuid, to_id: Uuid, arrow: Arrow) -> Result<(), &'static str> {
        let from_node = self.get_with_result(&from_id)?.linked.entry(arrow).or_default();
        if !from_node.contains(&to_id) {
            from_node.insert(to_id);
            self.get_with_result(&to_id)?.linked.entry(arrow.reverse()).or_default().insert(from_id);
        }
        Ok(())
    }

    fn get_with_result(&mut self, id: &Uuid) -> Result<&mut Node, &'static str> {
        self.nodes.get_mut(id).ok_or("Node ID not in graph.")
    }

    /// Disconnects one node on the graph from another with the given arrow type.
    pub fn disconnect(&mut self, from_id: Uuid, to_id: Uuid, arrow: Arrow) -> Result<(), &'static str> {
        if let Some(from_node) = self.get_with_result(&from_id)?.linked.get_mut(&arrow) && from_node.contains(&to_id) {
            from_node.remove(&to_id);
            self.get_with_result(&to_id)?.linked.get_mut(&arrow.reverse())
            .expect("CRITICAL: Bidirectional connect malfunctioning. PLEASE report to the GitHub repo.").remove(&from_id);
        }
        Ok(())
    }

    /// Traverses through the graph starting from a given node, through the arrow type specified. Uses Depth-First Search.
    pub fn traverse(&self, starting_id: Uuid, arrow: Arrow) -> impl Iterator<Item = Uuid> {
        let mut stack = vec![starting_id]; // Nodes to visit and track descendants
        let mut seen: HashSet<Uuid> = HashSet::from([starting_id]);

        std::iter::from_fn(move || {
            let node_id = stack.pop()?;
            if let Some(neighbors) = self.get(node_id).and_then(|node| node.linked.get(&arrow)) {
                for neighbor in neighbors.iter() {
                    if seen.insert(*neighbor) {
                        stack.push(*neighbor);
                    }
                }
            }
            Some(node_id)
        })
    }

    /// Deletes the node ID from the graph, deleting all direct connections without deleting any connected nodes.
    pub fn remove(&self, id: Uuid) -> Result<(), &'static str> {
        let node = &self.nodes.get(&id).ok_or("Node ID not in graph.")?;

        for neighbor in node.iter() {
            self.nodes.get(neighbor.1).ok_or("Neighbor ID not in graph.")?;
        }


        Ok(())
    }

    /// Finds all neighbors (direct links) of any given node ID in the graph 
    // pub fn find_neighbors(_node: Uuid, data) -> Vec<Uuid> {todo!()} TODO

    pub fn find_tree(id: Uuid) -> Vec<Uuid> {todo!()}
}

impl Default for Graph {fn default() -> Self {Self::new()}}

// impl From<HashMap<Uuid, Node>> for Graph {fn from(data: HashMap<Uuid, Node>) -> Self {Graph {nodes: data}}} enforce bidirectin

// TODO: Use parse to get data from JSON string.


impl std::ops::Index<Uuid> for Graph {
    type Output = Node;
    fn index(&self, id: Uuid) -> &Self::Output {&self.nodes[&id]}
}

impl std::fmt::Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {write!(f, "{:#?}", &self.nodes)}
}

impl<'a> IntoIterator for &'a Graph {
    type Item = &'a Node;
    type IntoIter = std::collections::hash_map::Values<'a, Uuid, Node>;

    fn into_iter(self) -> Self::IntoIter {self.nodes.values()}
}