use std::{collections::{HashMap, HashSet}, str::FromStr};
use uuid::Uuid;

use crate::{Arrow, DeebeeError::{self, *}, Node, Value};

pub struct Graph {
    nodes: HashMap<Uuid, Node>
}

// // // INITALIZATION // // //
impl Graph {
    /// Creates a new empty graph.
    pub fn new() -> Self {Graph {nodes: HashMap::new()}}

    /// Takes an owned HashMap and constructs a graph from it. Returns Err if references are not bidirectional.
    pub fn from_map(data: HashMap<Uuid, Node>) -> Result<Graph, DeebeeError> {Self::try_from(data)}

    pub fn from_json(data: &str) -> Result<Graph, DeebeeError> {Self::from_str(data)}
}

// // // UTILIZATION // // //
impl Graph {
    const BIDIRECTION_ERR: &str = "Bidirectional invariant malfunctioning. PLEASE report to the GitHub repo.";

    /// Gets node by UUID.
    pub fn get(&self, id: Uuid) -> Option<&Node> {self.nodes.get(&id)}

    /// Gets mutable node by UUID.
    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut Node> {self.nodes.get_mut(&id)}

    fn get_result(&self, id: Uuid) -> Result<&Node, DeebeeError> {self.nodes.get(&id).ok_or_else(|| NodeNotFound(id))}
    fn get_mut_result(&mut self, id: Uuid) -> Result<&mut Node, DeebeeError> {self.nodes.get_mut(&id).ok_or_else(|| NodeNotFound(id))}
    fn get_panic(&self, id: Uuid) -> &Node {    
        self.nodes.get(&id).expect("Node retrieval failed for unknown reason. PLEASE report to the GitHub repo.")
    }

    /// Checks if ID exists in graph.
    pub fn contains(&self, id: Uuid) -> bool {self.nodes.contains_key(&id)}

    /// Returns the number of nodes in the graph.
    pub fn len(&self) -> usize {self.nodes.len()}

    /// Returns `true` if the graph contains no nodes.
    pub fn is_empty(&self) -> bool {self.nodes.is_empty()}

    /// Returns an iterator through all the nodes of the graph.
    pub fn iter(&self) -> impl Iterator<Item = &Node> {self.nodes.values()}

    /// Returns an iterator of all nodes where the value matches the Value given.
    pub fn find(&self, data: impl Into<Value>) -> impl Iterator<Item = &Node> {
        let data = data.into();
        self.nodes.values().filter(move |node| node.data == data)
    }

    fn traverse_filtered(&self, starting_id: Uuid, filter: Option<&Arrow>) -> Result<impl Iterator<Item = &Node>, DeebeeError> {
        let mut stack = vec![self.get_result(starting_id)?]; // Nodes to visit and track descendants
        let mut seen: HashSet<Uuid> = HashSet::from([starting_id]);

        Ok(std::iter::from_fn(move || {
            let node = stack.pop()?;
            for (arrow, id) in node {
                if filter.map_or(true, |filtered_arrow| filtered_arrow == arrow) && seen.insert(*id) {                                                                                         
                    stack.push(self.get_panic(*id));                                                                                                                                          
                }  
            }
            Some(node)
        }))
    }

    /// Traverses through the graph starting from a given node, through all arrow types. Uses Depth-First Search.
    pub fn traverse_all(&self, starting_id: Uuid) -> Result<impl Iterator<Item = &Node>, DeebeeError> {
        self.traverse_filtered(starting_id, None)
    }

    /// Traverses through the graph starting from a given node, through the specified arrow type. Uses Depth-First Search.
    pub fn traverse(&self, starting_id: Uuid, arrow: &Arrow) -> Result<impl Iterator<Item = &Node>, DeebeeError> {
        self.traverse_filtered(starting_id, Some(&arrow))
    }

    /// Gets all neighbors of a given node, through all arrow types. If you only need IDs, use `Node.iter()` instead.
    pub fn all_neighbors(&self, starting_id: Uuid) -> Result<impl Iterator<Item = (&Arrow, &Node)>, DeebeeError> {
        Ok(self.get_result(starting_id)?.iter().map(|(arrow, id)| (arrow, self.get_panic(*id))))
    }

    /// Gets all neighbors of a given node, through the specified arrow type. If you only need IDs, use `Node.iter()` instead.
    pub fn neighbors(&self, starting_id: Uuid, arrow: &Arrow) -> Result<impl Iterator<Item = &Node>, DeebeeError> {
        Ok(self.get_result(starting_id)?.iter().filter_map(move |(neighbor_arr, id)| {
            (neighbor_arr == arrow).then(|| self.get_panic(*id))
        }))
    }
    
    /// Returns an iterator of all neighbors of the node through the specified arrrow type given where the value matches the Value given.
    pub fn find_neighbors(&self, id: Uuid, data: impl Into<Value>, arrow: &Arrow) -> Result<impl Iterator<Item = &Node>, DeebeeError> {
        let data = data.into();
        Ok(self.neighbors(id, arrow)?.filter(move |node| node.data == data))
    }

    /// Returns an iterator of all descendants of the node through the specified arrrow type given where the value matches the Value given.
    pub fn find_tree(&self, id: Uuid, data: impl Into<Value>, arrow: &Arrow) -> Result<impl Iterator<Item = &Node>, DeebeeError> {
        let data = data.into();
        Ok(self.traverse(id, arrow)?.filter(move |node| node.data == data))
    }
}

// // // MODIFICATION // // //
impl Graph {
    /// Inserts a new node to the graph.
    pub fn add(&mut self, data: impl Into<Value>) -> &Node {
        let node = Node::new(data);
        let id = node.id;
        self.nodes.insert(id, node);
        self.get_panic(id)
    }

    /// Inserts an already initialized node into the Graph. Unlike other methods, insert takes a full Node rather than a Uuid.
    pub fn insert(&mut self, node: Node) -> &Node {
        let id = node.id;
        self.nodes.insert(id, node);
        self.get_panic(id)
    }

    /// Extends graph from the given node map.
    pub fn extend(&mut self, extension: impl IntoIterator<Item = (Uuid, Node)>) {self.nodes.extend(extension)}

    /// Connects one node on the graph to another using the given arrow type.
    pub fn connect(&mut self, from_id: Uuid, to_id: Uuid, arrow: &Arrow) -> Result<(), DeebeeError> {
        let from_node = self.get_mut_result(from_id)?.linked.entry(arrow.clone()).or_default();
        if !from_node.contains(&to_id) {
            from_node.insert(to_id);
            self.get_mut_result(to_id)?.linked.entry(arrow.reverse()).or_default().insert(from_id);
        }
        Ok(())
    }

    /// Disconnects one node on the graph from another with the given arrow type.
    pub fn disconnect(&mut self, from_id: Uuid, to_id: Uuid, arrow: &Arrow) -> Result<(), DeebeeError> {
        if let Some(from_node) = self.get_mut_result(from_id)?.linked.get_mut(&arrow) && from_node.contains(&to_id) {
            from_node.remove(&to_id);
            self.get_mut_result(to_id)?.linked.get_mut(&arrow.reverse())
            .expect(Self::BIDIRECTION_ERR).remove(&from_id);
        }

        Ok(())
    }

    /// Clears entire graph.
    pub fn clear(&mut self) {self.nodes.clear()}

    /// Deletes the node ID from the graph, deleting all direct connections without deleting any connected nodes.
    pub fn remove(&mut self, id: Uuid) -> Result<(), DeebeeError> {
        let node = self.nodes.remove(&id).ok_or_else(|| NodeNotFound(id))?;
        for (arrow, neighbor) in node.iter() {
            let neighbor = self.nodes.get_mut(neighbor).expect(Self::BIDIRECTION_ERR);
            neighbor.linked.get_mut(&arrow.reverse()).expect(Self::BIDIRECTION_ERR).remove(&id);
        }
        Ok(())
    }
}

// // // EXPORTATION // // //
impl Graph {
    /// Returns map of all nodes in the graph.
    pub fn to_map(&self) -> &HashMap<Uuid, Node> {&self.nodes}

    /// Writes the graph in the format of a JSON string into a specified buffer.
    pub fn write_json(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(w, "{{\n")?;

        for (node_number, (id, node)) in self.nodes.iter().enumerate() { // Write node object
            if node_number > 0 {write!(w, ",\n")?;}

            write!(w, "    \"{id}\": {{\
                     \n        \"data\": {},\
                     \n        \"linked\": [\
                     \n            ", node.data.serialize())?;

            for (linked_number, (arrow, ids)) in node.linked.iter().enumerate() { // Write each linked HashSet
                if linked_number > 0 { write!(w, ",\n            ")?; }
                write!(w, r#"{{"label": "{}", "reverse": "{}", "neighbors": ["#, arrow.label, arrow.reverse_label)?;

                for (id_number, id) in ids.iter().enumerate() { // Write neighbors list
                    if id_number > 0 { write!(w, ", ")?; }
                    write!(w, "\"{id}\"")?;
                }

                write!(w, "]}}")?;
            }
            write!(w, "\n        ]\n    }}")?;
        }

        write!(w, "\n}}")?;
        Ok(())
    }

    /// Returns JSON-like String of all nodes in the graph. For greater performance, use `write_json` and pass your own buffer.
    pub fn to_json(&self) -> String {
        let mut writer = String::new();
        self.write_json(&mut writer).unwrap();
        writer
    }
}

impl Default for Graph {fn default() -> Self {Self::new()}}

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

impl std::str::FromStr for Graph {
    type Err = DeebeeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string = s.strip_prefix('{').and_then(|s| s.strip_suffix('}')).ok_or_else(|| InvalidJson("JSON not an object at top-level.".to_string()))?;

        let mut parsed: HashMap<Uuid, Node> = HashMap::new();
        for (id_str, node) in Value::parse_collection(string)?.into_iter() {
            let id = id_str.parse::<Uuid>()?;

            let Value::Map(mut node) = node else {
                return Err(InvalidCreation(format!("Node {id_str} data is not a JSON object (aka dictionary/HashMap).")));
            };

            let data = node.remove("data").ok_or_else(|| InvalidCreation(format!("{id_str} does not have data.")))?;

            let linked_nodes = node.remove("linked").unwrap_or_else(|| Value::List(vec![]));
            let Value::List(linked_nodes) = linked_nodes else {return Err(InvalidCreation(format!("Node {id_str}'s linked nodes is not an array.")))};

            let mut linked: HashMap<Arrow, HashSet<Uuid>> = HashMap::new();

            for arrow_val in linked_nodes {
                let Value::Map(mut arrow) = arrow_val else {return Err(InvalidCreation(format!("One of node {id_str}'s links is not a map.")))};

                let label = arrow.remove("label").ok_or_else(|| InvalidCreation(format!("Node {id_str}'s linked nodes does not have a label.")))?;
                let Value::Text(label) = label else {return Err(InvalidCreation(format!("One of node {id_str}'s arrow labels is not text.")))};

                let reverse = arrow.remove("reverse").ok_or_else(|| InvalidCreation(format!("Node {id_str}'s linked nodes does not have a reverse label.")))?;
                let Value::Text(reverse) = reverse else {return Err(InvalidCreation(format!("One of node {id_str}'s arrow reverse labels is not text.")))};

                let neighbors = arrow.remove("neighbors").unwrap_or_else(|| Value::List(vec![]));
                let Value::List(neighbors) = neighbors else {return Err(InvalidCreation(format!("Node {id_str}'s linked nodes is not an array.")))};

                let key = Arrow::new(label, reverse);
                let neighbor_set = neighbors.iter().filter_map(|n| if let Value::Text(s) = n { s.parse::<Uuid>().ok() } else { None }).collect::<HashSet<Uuid>>();
                linked.insert(key, neighbor_set);
            }

            parsed.insert(id, Node {id, data, linked});
        };

        Ok(Self::try_from(parsed)?)
    }
}

impl TryFrom<HashMap<Uuid, Node>> for Graph {
    type Error = DeebeeError;

    fn try_from(value: HashMap<Uuid, Node>) -> Result<Self, Self::Error> {
        let mut seen: HashSet<(Uuid, Uuid, Arrow)> = HashSet::new();
        
        for (id, node) in &value {
            for (arrow, linked) in node.iter() {
                if seen.contains(&(*id, *linked, arrow.clone())) {continue}

                let linked_node = value.get(linked).ok_or_else(|| InvalidCreation(format!("Linked ID {linked} not in graph.")))?;

                if !linked_node.linked.get(&arrow.reverse()).is_some_and(|nodes| nodes.contains(&id)) {
                    return Err(InvalidCreation(format!("Bidirectionality of arrows not enforced: {id} links with {} (reverse {}) to {linked}, \
                    but {linked} does not link to {id} through {}", arrow.label, arrow.reverse_label, arrow.reverse_label)));
                }
                seen.insert((*linked, *id, arrow.reverse()));
                
            }
        }
        Ok(Graph {nodes: value})
    }
}
