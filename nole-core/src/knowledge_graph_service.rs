// Knowledge graph service
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeNode {
    pub id: String,
    pub label: String,
    pub mastery_level: u8,
    pub subject: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEdge {
    pub source: String,
    pub target: String,
    pub strength: f32, // 0.0-1.0
    pub relationship_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    pub nodes: Vec<KnowledgeNode>,
    pub edges: Vec<KnowledgeEdge>,
}

pub struct KnowledgeGraphService {
    graph: KnowledgeGraph,
}

impl KnowledgeGraphService {
    pub fn new() -> Self {
        Self {
            graph: KnowledgeGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
            },
        }
    }

    pub fn get_graph(&self) -> KnowledgeGraph {
        self.graph.clone()
    }

    pub fn add_node(&mut self, node: KnowledgeNode) {
        self.graph.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: KnowledgeEdge) {
        self.graph.edges.push(edge);
    }
}

impl Default for KnowledgeGraphService {
    fn default() -> Self {
        Self::new()
    }
}
