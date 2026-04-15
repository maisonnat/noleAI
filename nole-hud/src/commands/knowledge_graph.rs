use nole_core::knowledge_graph_service::{KnowledgeGraph, KnowledgeGraphService, KnowledgeNode, KnowledgeEdge};
use nole_core::vault::VaultParser;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct KnowledgeGraphState {
    pub service: Arc<Mutex<KnowledgeGraphService>>,
    pub vault_parser: Option<Arc<VaultParser>>,
}

#[derive(serde::Serialize)]
pub struct KnowledgeGraphResponse {
    pub nodes: Vec<KnowledgeGraphNode>,
    pub edges: Vec<KnowledgeGraphEdge>,
}

#[derive(serde::Serialize)]
pub struct KnowledgeGraphNode {
    pub id: String,
    pub label: String,
    pub mastery_level: u8,
    pub subject: String,
}

#[derive(serde::Serialize)]
pub struct KnowledgeGraphEdge {
    pub source: String,
    pub target: String,
    pub strength: f32,
    pub relationship_type: String,
}

pub async fn get_knowledge_graph(state: KnowledgeGraphState) -> Result<KnowledgeGraphResponse, String> {
    let graph = if let Some(parser) = &state.vault_parser {
        let subjects = parser.parse_config().map_err(|e| e.to_string())?;
        let subject_data: Vec<(String, u8, Vec<String>)> = subjects
            .iter()
            .map(|s| (s.name.clone(), s.mastery_level, s.topics.clone()))
            .collect();
        KnowledgeGraphService::generate_from_subjects(&subject_data)
    } else {
        let service = state.service.lock().await;
        service.get_graph()
    };

    Ok(KnowledgeGraphResponse {
        nodes: graph.nodes.into_iter().map(|n| KnowledgeGraphNode {
            id: n.id,
            label: n.label,
            mastery_level: n.mastery_level,
            subject: n.subject,
        }).collect(),
        edges: graph.edges.into_iter().map(|e| KnowledgeGraphEdge {
            source: e.source,
            target: e.target,
            strength: e.strength,
            relationship_type: e.relationship_type,
        }).collect(),
    })
}
