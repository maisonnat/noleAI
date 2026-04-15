use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A node in the knowledge graph representing a subject or topic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeNode {
    pub id: String,
    pub label: String,
    pub mastery_level: u8,
    pub subject: String,
}

/// A directed edge between two knowledge nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEdge {
    pub source: String,
    pub target: String,
    /// Relationship strength from 0.0 to 1.0.
    pub strength: f32,
    pub relationship_type: String,
}

/// The full knowledge graph with nodes and edges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    pub nodes: Vec<KnowledgeNode>,
    pub edges: Vec<KnowledgeEdge>,
}

/// Service for building and querying knowledge graphs.
///
/// Maintains an indexed representation for fast lookups by node ID.
pub struct KnowledgeGraphService {
    graph: KnowledgeGraph,
    node_index: HashMap<String, usize>,
    adjacency: HashMap<String, Vec<usize>>,
}

impl KnowledgeGraphService {
    /// Create an empty knowledge graph service.
    pub fn new() -> Self {
        Self {
            graph: KnowledgeGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
            },
            node_index: HashMap::new(),
            adjacency: HashMap::new(),
        }
    }

    /// Get a clone of the current graph data.
    pub fn get_graph(&self) -> KnowledgeGraph {
        self.graph.clone()
    }

    /// Add a node to the graph, updating the index.
    pub fn add_node(&mut self, node: KnowledgeNode) {
        let idx = self.graph.nodes.len();
        self.node_index.insert(node.id.clone(), idx);
        self.graph.nodes.push(node);
    }

    /// Add an edge, updating the adjacency list.
    pub fn add_edge(&mut self, edge: KnowledgeEdge) {
        if let Some(&src_idx) = self.node_index.get(&edge.source) {
            self.adjacency
                .entry(edge.source.clone())
                .or_default()
                .push(src_idx);
        }
        self.graph.edges.push(edge);
    }

    /// Find a node by its ID in O(1) via the index.
    pub fn find_node(&self, id: &str) -> Option<&KnowledgeNode> {
        self.node_index.get(id).map(|&idx| &self.graph.nodes[idx])
    }

    /// Get all edges connected to a node in O(k) where k = edge count for that node.
    pub fn get_connected_edges(&self, node_id: &str) -> Vec<&KnowledgeEdge> {
        self.graph
            .edges
            .iter()
            .filter(|e| e.source == node_id || e.target == node_id)
            .collect()
    }

    /// Generate a knowledge graph from subject data tuples.
    ///
    /// Each tuple is (name, mastery_level, topics).
    /// Creates subject nodes, topic nodes, "contains" edges, and cross-subject "related" edges.
    pub fn generate_from_subjects(subjects: &[(String, u8, Vec<String>)]) -> KnowledgeGraph {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut subject_ids = Vec::new();

        for (name, mastery, topics) in subjects {
            let subject_id = name.to_lowercase().replace(' ', "_");
            nodes.push(KnowledgeNode {
                id: subject_id.clone(),
                label: name.clone(),
                mastery_level: *mastery,
                subject: name.clone(),
            });
            subject_ids.push(subject_id.clone());

            for topic in topics {
                let topic_id = format!("{}_{}", subject_id, topic.to_lowercase().replace(' ', "_"));
                nodes.push(KnowledgeNode {
                    id: topic_id.clone(),
                    label: topic.clone(),
                    mastery_level: (*mastery).saturating_sub(1).max(1),
                    subject: name.clone(),
                });
                edges.push(KnowledgeEdge {
                    source: subject_id.clone(),
                    target: topic_id,
                    strength: 0.8,
                    relationship_type: "contains".to_string(),
                });
            }
        }

        if subject_ids.len() > 1 {
            for i in 0..subject_ids.len().saturating_sub(1) {
                edges.push(KnowledgeEdge {
                    source: subject_ids[i].clone(),
                    target: subject_ids[i + 1].to_string(),
                    strength: 0.3,
                    relationship_type: "related".to_string(),
                });
            }
        }

        KnowledgeGraph { nodes, edges }
    }
}

impl Default for KnowledgeGraphService {
    fn default() -> Self {
        Self::new()
    }
}
