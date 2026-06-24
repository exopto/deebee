use std::{collections::{HashMap, HashSet}};
use uuid::Uuid;
use crate::{Node, Value, Arrow};
use std::str::FromStr;

pub struct Graph {
    nodes: HashMap<Uuid, Node>
}

// // // INITALIZATION // // //
impl Graph {
    /// Creates a new empty graph.
    pub fn new() -> Self {Graph {nodes: HashMap::new()}}

    /// Takes an owned HashMap and constructs a graph from it. Returns Err if references are not bidirectional.
    pub fn from_map(data: HashMap<Uuid, Node>) -> Result<Graph, String> {Self::try_from(data)}

    pub fn from_json(data: &str) -> Result<Graph, String> {Self::from_str(data)}
}

// // // UTILIZATION // // //
impl Graph {
    const BIDIRECTION_ERR: &str = "Bidirectional invariant malfunctioning. PLEASE report to the GitHub repo.";

    /// Gets node by UUID.
    pub fn get(&self, id: Uuid) -> Option<&Node> {self.nodes.get(&id)}

    /// Gets mutable node by UUID.
    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut Node> {self.nodes.get_mut(&id)}

    fn get_mut_result(&mut self, id: Uuid) -> Result<&mut Node, &'static str> {
        self.nodes.get_mut(&id).ok_or("Node ID not in graph.")
    }

    // fn get_result(&self, id: Uuid) -> Result<&Node, &'static str> {
    //     self.nodes.get(&id).ok_or("Node ID not in graph.")
    // }

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

    /// Traverses through the graph starting from a given node, through all arrow types. Uses Depth-First Search.
    pub fn traverse(&self, starting_id: Uuid) -> impl Iterator<Item = Uuid> {
        let mut stack = vec![starting_id]; // Nodes to visit and track descendants
        let mut seen: HashSet<Uuid> = HashSet::from([starting_id]);

        std::iter::from_fn(move || {
            let node = self.get_panic(stack.pop()?);
            // if let Some(neighbors) = node.linked.get(&arrow) {
                for (_, id) in node {
                    if seen.insert(*id) {
                        stack.push(*id);
                    }
                }
            // }
            Some(node.id)
        })
    }

    // pub fn traverse_arrow(&self, starting_id: Uuid, arrow: &Arrow) -> impl Iterator<Item = Uuid> {
    //     // self.traverse(starting_id).filter(|id| self.get_panic(*id).linked.get(&arrow).unwrap_or(HashSet::new).contains())
    //     return self.traverse(starting_id); // TODO
    // }

    /// Returns an iterator of all nodes where the value matches the Value given.
    pub fn find(&self, data: impl Into<Value>) -> impl Iterator<Item = &Node> {
        let data = data.into();
        self.nodes.values().filter(move |node| node.data == data)
    }

    /// Returns an iterator of all neighbors of the node given where the value matches the Value given. Returns None if ID is not in graph.
    pub fn find_neighbors(&self, id: Uuid, data: impl Into<Value>) -> Option<impl Iterator<Item = (&Arrow, &Uuid)>> {
        let data = data.into();
        Some(self.nodes.get(&id)?.iter().filter(move |(_, id)| self.get_panic(**id).data == data))
    }

    /// Returns an iterator of all descendants of the node given where the value matches the Value given. Returns None if ID is not in graph.:
   pub fn find_tree(&self, id: Uuid, data: impl Into<Value>) -> Option<impl Iterator<Item = Uuid>> {
        let data = data.into();
        Some(self.traverse(id).filter(move |id| self.get_panic(*id).data == data))
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
    pub fn connect(&mut self, from_id: Uuid, to_id: Uuid, arrow: &Arrow) -> Result<(), &'static str> {
        let from_node = self.get_mut_result(from_id)?.linked.entry(arrow.clone()).or_default();
        if !from_node.contains(&to_id) {
            from_node.insert(to_id);
            self.get_mut_result(to_id)?.linked.entry(arrow.reverse()).or_default().insert(from_id);
        }
        Ok(())
    }

    /// Disconnects one node on the graph from another with the given arrow type.
    pub fn disconnect(&mut self, from_id: Uuid, to_id: Uuid, arrow: &Arrow) -> Result<(), &'static str> {
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
    pub fn remove(&mut self, id: Uuid) -> Result<(), &'static str> {        
        for neighbor in self.nodes.remove(&id).iter() {
            let neighbor = self.nodes.get_mut(&neighbor.id).expect(Self::BIDIRECTION_ERR);
            for neighbor_linked in neighbor.linked.values_mut() {
                neighbor_linked.remove(&id);
            }
        }
        
        Ok(())
    }
}


// // // EXPORTATION // // //
impl Graph {
    /// Returns map of all nodes in the graph.
    pub fn to_map(&self) -> &HashMap<Uuid, Node> {&self.nodes}

    /// Returns JSON-like String of all nodes in the graph.
    pub fn to_json(&self) -> String {
        let nodes = self.nodes.iter().map(|(id, node)| {

            let linked = node.linked.iter().map(|(arrow, ids)| {
                format!(
                    r#"{{"label": "{}", "reverse": "{}", "neighbors": [{}]}}"#, 
                    arrow.label, 
                    arrow.reverse_label,
                    ids.iter().map(|id| format!("\"{}\"", id.to_string())).collect::<Vec<String>>().join(", ")
                )
            }).collect::<Vec<String>>().join(",\n            ");

            format!(
                 "    \"{id}\": {{\
                \n        \"data\": {},\
                \n        \"linked\": [\
                \n            {linked}\
                \n        ]\
                \n    }}",
                node.data.serialize()
            )

        }).collect::<Vec<String>>().join(",\n");

        format!("{{\n{}\n}}", nodes)
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static DEFAULT_VEC: Value = Value::List(vec![]);
        let string = s.strip_prefix('{').and_then(|s| s.strip_suffix('}')).ok_or("JSON not an object at top-level.")?;

        let mut parsed: HashMap<Uuid, Node> = HashMap::new();
        for (id, node) in Value::parse_collection(string)?.into_iter() {
            let id = id.parse::<Uuid>().map_err(|_| "Invalid UUID format.")?;

            let Value::Map(raw_node) = node else {
                return Err(format!("Node {id} data is not a JSON object (aka dictionary/HashMap)."));
            };

            let data = raw_node.get("data").cloned().ok_or(format!("{id} does not have data."))?;
            let linked_nodes = raw_node.get("linked").cloned().unwrap_or(Value::List(vec![]));
            let Value::List(linked_nodes) = linked_nodes else {
                return Err(format!("Node {id}'s linked nodes is not an array."));
            };

            let mut linked: HashMap<Arrow, HashSet<Uuid>> = HashMap::new();

            for arrow in linked_nodes {
                let Value::Map(arrow) = arrow else {
                    return Err(format!("One of node {id}'s links is not a map."));
                };

                let label = arrow.get("label").cloned().ok_or(format!("Node {id}'s linked nodes does not have a label."))?;
                let Value::Text(label) = label else {
                    return Err(format!("One of node {id}'s arrow labels is not text."));
                };

                let reverse = arrow.get("reverse").cloned().ok_or(format!("Node {id}'s linked nodes does not have a reverse label."))?;
                let Value::Text(reverse) = reverse else {
                    return Err(format!("One of node {id}'s arrow reverse labels is not text."));
                };

                let neighbors = arrow.get("neighbors").unwrap_or(&DEFAULT_VEC);
                let Value::List(neighbors) = neighbors else {
                    return Err(format!("Node {id}'s linked nodes is not an array."));
                };

                let key = Arrow::new(label, reverse);
                let neighbor_set = neighbors.iter()
                    .filter_map(|n| if let Value::Text(s) = n { s.parse::<Uuid>().ok() } else { None })
                    .collect::<HashSet<Uuid>>();
                linked.insert(key, neighbor_set);
            }

            parsed.insert(id, Node {id, data, linked});

            
            
        };

        Ok(Self::try_from(parsed)?)

    }
}

impl TryFrom<HashMap<Uuid, Node>> for Graph {
    type Error = String;

    fn try_from(value: HashMap<Uuid, Node>) -> Result<Self, Self::Error> {
        let mut seen: HashSet<(Uuid, Uuid, Arrow)> = HashSet::new();
        
        for (id, node) in &value {
            for (arrow, linked) in node.iter() {
                if seen.contains(&(*id, *linked, arrow.clone())) {continue}

                let linked_node = value.get(linked).ok_or_else(|| format!("Linked ID {linked} not in graph."))?;

                if !linked_node.linked.get(&arrow.reverse()).is_some_and(|nodes| nodes.contains(&id)) {
                    return Err(format!("Bidirectionality of arrows not enforced: {id} links with {} (reverse {}) to {linked}, \
                    but {linked} does not link to {id} through {}", arrow.label, arrow.reverse_label, arrow.reverse_label));
                }
                seen.insert((*linked, *id, arrow.reverse()));
                
            }
        }
        Ok(Graph {nodes: value})
    }
    
}
