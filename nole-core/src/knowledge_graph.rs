use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeNode {
    pub id: String,
    pub label: String,
    pub topic: String,
    pub mastery_level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEdge {
    pub from: String,
    pub to: String,
    pub relation: String, // ej: "requiere", "relacionado con", "parte de"
    pub strength: f32,    // 0.0 - 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    pub nodes: Vec<KnowledgeNode>,
    pub edges: Vec<KnowledgeEdge>,
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: KnowledgeNode) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: KnowledgeEdge) {
        self.edges.push(edge);
    }

    pub fn find_connections(&self, node_id: &str) -> Vec<&KnowledgeEdge> {
        self.edges
            .iter()
            .filter(|edge| edge.from == node_id || edge.to == node_id)
            .collect()
    }

    pub fn get_dependencies(&self, node_id: &str) -> Vec<&KnowledgeNode> {
        let dependencies: HashSet<String> = self
            .edges
            .iter()
            .filter(|edge| edge.to == node_id && edge.relation == "requiere")
            .map(|edge| edge.from.clone())
            .collect();

        self.nodes
            .iter()
            .filter(|node| dependencies.contains(&node.id))
            .collect()
    }

    pub fn calculate_complexity(&self, node_id: &str) -> f32 {
        let mut visited = HashSet::new();

        fn dfs(
            graph: &KnowledgeGraph,
            node_id: &str,
            visited: &mut HashSet<String>,
            depth: u32,
        ) -> u32 {
            if visited.contains(node_id) || depth > 10 {
                return 0;
            }

            visited.insert(node_id.to_string());

            let mut total_depth = depth;
            for edge in &graph.edges {
                if edge.from == node_id {
                    total_depth += dfs(graph, &edge.to, visited, depth + 1);
                }
            }

            total_depth
        }

        dfs(self, node_id, &mut visited, 0) as f32
    }

    pub fn generate_recommendations(&self, node_id: &str) -> Vec<String> {
        let mut recommendations = Vec::new();
        let dependencies = self.get_dependencies(node_id);

        if dependencies.is_empty() {
            recommendations.push(
                "No hay prerequisitos detectados. Puedes comenzar con este tema.".to_string(),
            );
        } else {
            recommendations.push("Prerequisitos identificados:".to_string());
            for dep in dependencies {
                recommendations.push(format!(
                    "- {}: Nivel de maestría {:.0}% (requiere refuerzo)",
                    dep.label, dep.mastery_level
                ));
            }
        }

        let connections = self.find_connections(node_id);
        let complexity = self.calculate_complexity(node_id);

        if complexity > 20.0 {
            recommendations.push(
                "⚠️ Este tema es complejo. Considera dividirlo en subtemas más pequeños."
                    .to_string(),
            );
        }

        let strong_connections: Vec<_> = connections
            .iter()
            .filter(|edge| edge.strength > 0.7)
            .collect();

        if !strong_connections.is_empty() {
            recommendations.push("💡 Temas fuertemente relacionados:".to_string());
            for edge in strong_connections {
                let target = if edge.from == node_id {
                    &edge.to
                } else {
                    &edge.from
                };
                if let Some(node) = self.nodes.iter().find(|n| n.id == *target) {
                    recommendations.push(format!("- {} ({})", node.label, edge.relation));
                }
            }
        }

        recommendations
    }

    pub fn to_visualization_format(&self) -> String {
        let mut dot = String::from("digraph KnowledgeGraph {\n");
        dot.push_str("  rankdir=LR;\n");
        dot.push_str("  node [shape=box, style=rounded];\n\n");

        // Add nodes with colors based on mastery level
        for node in &self.nodes {
            let color = if node.mastery_level >= 80.0 {
                "#90EE90" // light green
            } else if node.mastery_level >= 50.0 {
                "#FFD700" // gold
            } else {
                "#FFB6C1" // light pink
            };

            dot.push_str(&format!(
                "  \"{}\" [label=\"{}\\nMaestría: {:.0}%\", fillcolor=\"{}\", style=\"filled,rounded\"];\n",
                node.id, node.label, node.mastery_level, color
            ));
        }

        dot.push_str("\n");

        // Add edges
        for edge in &self.edges {
            let style = if edge.strength > 0.7 {
                "bold"
            } else if edge.strength > 0.4 {
                "solid"
            } else {
                "dashed"
            };

            dot.push_str(&format!(
                "  \"{}\" -> \"{}\" [label=\"{}\", style={}, weight={:.1}];\n",
                edge.from,
                edge.to,
                edge.relation,
                style,
                edge.strength * 10.0
            ));
        }

        dot.push_str("}\n");
        dot
    }

    pub fn find_learning_path(&self, start_node: &str, target_node: &str) -> Option<Vec<String>> {
        let mut visited = HashSet::new();
        let mut path = Vec::new();

        if self.find_path_recursive(start_node, target_node, &mut visited, &mut path) {
            Some(path)
        } else {
            None
        }
    }

    fn find_path_recursive(
        &self,
        current: &str,
        target: &str,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> bool {
        if current == target {
            path.push(target.to_string());
            return true;
        }

        visited.insert(current.to_string());
        path.push(current.to_string());

        for edge in &self.edges {
            if edge.from == current && !visited.contains(&edge.to) {
                if self.find_path_recursive(&edge.to, target, visited, path) {
                    return true;
                }
            }
        }

        path.pop();
        false
    }
}

impl Default for KnowledgeGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knowledge_graph_creation() {
        let mut graph = KnowledgeGraph::new();

        let node1 = KnowledgeNode {
            id: "math".to_string(),
            label: "Matemáticas".to_string(),
            topic: "Matemáticas".to_string(),
            mastery_level: 75.0,
        };

        let node2 = KnowledgeNode {
            id: "calculus".to_string(),
            label: "Cálculo".to_string(),
            topic: "Matemáticas".to_string(),
            mastery_level: 45.0,
        };

        graph.add_node(node1);
        graph.add_node(node2);

        assert_eq!(graph.nodes.len(), 2);
    }

    #[test]
    fn test_add_edge() {
        let mut graph = KnowledgeGraph::new();

        let edge = KnowledgeEdge {
            from: "math".to_string(),
            to: "calculus".to_string(),
            relation: "requiere".to_string(),
            strength: 0.9,
        };

        graph.add_edge(edge);
        assert_eq!(graph.edges.len(), 1);
    }

    #[test]
    fn test_find_connections() {
        let mut graph = KnowledgeGraph::new();

        graph.add_node(KnowledgeNode {
            id: "math".to_string(),
            label: "Matemáticas".to_string(),
            topic: "Matemáticas".to_string(),
            mastery_level: 75.0,
        });

        graph.add_edge(KnowledgeEdge {
            from: "math".to_string(),
            to: "calculus".to_string(),
            relation: "requiere".to_string(),
            strength: 0.9,
        });

        let connections = graph.find_connections("math");
        assert_eq!(connections.len(), 1);
    }

    #[test]
    fn test_generate_recommendations() {
        let mut graph = KnowledgeGraph::new();

        graph.add_node(KnowledgeNode {
            id: "math".to_string(),
            label: "Matemáticas".to_string(),
            topic: "Matemáticas".to_string(),
            mastery_level: 30.0,
        });

        graph.add_node(KnowledgeNode {
            id: "algebra".to_string(),
            label: "Álgebra".to_string(),
            topic: "Matemáticas".to_string(),
            mastery_level: 40.0,
        });

        graph.add_edge(KnowledgeEdge {
            from: "algebra".to_string(),
            to: "math".to_string(),
            relation: "requiere".to_string(),
            strength: 0.8,
        });

        let recommendations = graph.generate_recommendations("math");
        assert!(!recommendations.is_empty());
        assert!(recommendations.iter().any(|r| r.contains("Prerequisitos")));
    }

    #[test]
    fn test_find_learning_path() {
        let mut graph = KnowledgeGraph::new();

        graph.add_node(KnowledgeNode {
            id: "algebra".to_string(),
            label: "Álgebra".to_string(),
            topic: "Matemáticas".to_string(),
            mastery_level: 80.0,
        });

        graph.add_node(KnowledgeNode {
            id: "calculus".to_string(),
            label: "Cálculo".to_string(),
            topic: "Matemáticas".to_string(),
            mastery_level: 50.0,
        });

        graph.add_node(KnowledgeNode {
            id: "analysis".to_string(),
            label: "Análisis".to_string(),
            topic: "Matemáticas".to_string(),
            mastery_level: 30.0,
        });

        graph.add_edge(KnowledgeEdge {
            from: "algebra".to_string(),
            to: "calculus".to_string(),
            relation: "requiere".to_string(),
            strength: 0.9,
        });

        graph.add_edge(KnowledgeEdge {
            from: "calculus".to_string(),
            to: "analysis".to_string(),
            relation: "requiere".to_string(),
            strength: 0.9,
        });

        let path = graph.find_learning_path("algebra", "analysis");
        assert!(path.is_some());
        assert_eq!(path.unwrap().len(), 3);
    }
}
