use markdown_ast::{markdown_to_ast, Block, Inline, Inlines};
use std::fs;
use std::collections::HashMap;
use serde_yaml;

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_markdown_file(path: &str) -> (HashMap<String, serde_yaml::Value>, String) {
        let content = fs::read_to_string(path).unwrap();
        let ast = markdown_to_ast(&content);
        
        let mut metadata = String::new();
        let mut body = String::new();
        
        for block in ast {
            match block {
                Block::Paragraph(inlines) => {
                    let text = inlines_to_string(inlines);
                    if text.starts_with("---") && metadata.is_empty() {
                        // This is the YAML front matter
                        metadata.push_str(&text);
                    } else {
                        body.push_str(&text);
                        body.push('\n');
                    }
                }
                Block::Heading(_, inlines) => {
                    body.push_str(&inlines_to_string(inlines));
                    body.push('\n');
                }
                _ => {}
            }
        }
        
        let metadata: HashMap<String, serde_yaml::Value> = serde_yaml::from_str(&metadata)
            .unwrap_or_else(|_| HashMap::new());
            
        (metadata, body)
    }

    fn inlines_to_string(inlines: Inlines) -> String {
        let mut result = String::new();
        for inline in inlines.0 {
            match inline {
                Inline::Text(text) => result.push_str(&text),
                Inline::Strong(inlines) => result.push_str(&inlines_to_string(inlines)),
                Inline::Emphasis(inlines) => result.push_str(&inlines_to_string(inlines)),
                Inline::Code(text) => result.push_str(&text),
                Inline::SoftBreak => result.push('\n'),
                Inline::HardBreak => result.push('\n'),
                _ => result.push_str(""), // Ignore other variants
            }
        }
        result
    }

    #[test]
    fn test_happy_path_parsing() {
        let path = "examples/loan_approval.md";
        let (metadata, _) = parse_markdown_file(path);
        
        assert!(metadata.contains_key("dmn"));
        let dmn = metadata.get("dmn").unwrap();
        
        assert_eq!(dmn.get("id").and_then(|v| v.as_str()), Some("D1"));
        
        let requires = dmn.get("requires")
            .and_then(|v| v.as_sequence())
            .unwrap()
            .iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<&str>>();
            
        assert!(requires.contains(&"checks/income.md#thresholds"));
        assert!(requires.contains(&"inputs/credit_score.md"));
    }

    #[test]
    fn test_metadata_presence() {
        let files = [
            "examples/loan_approval.md",
            "examples/checks/income.md",
            "examples/inputs/credit_score.md",
            "examples/api/approval_service.md"
        ];
        
        for file in files {
            let (metadata, _) = parse_markdown_file(file);
            assert!(metadata.contains_key("dmn"), "Missing DMN metadata in {}", file);
            let dmn = metadata.get("dmn").unwrap();
            assert!(dmn.get("id").is_some(), "Missing id in {}", file);
        }
    }

    #[test]
    fn test_link_validation() {
        let files = [
            ("examples/loan_approval.md", vec![
                "checks/income.md",
                "inputs/credit_score.md"
            ]),
            ("examples/checks/income.md", vec![
                "loan_approval.md"
            ]),
            ("examples/inputs/credit_score.md", vec![
                "loan_approval.md"
            ]),
            ("examples/api/approval_service.md", vec![
                "loan_approval.md"
            ])
        ];
        
        for (file, expected_links) in files {
            let (_, body) = parse_markdown_file(file);
            
            for link in expected_links {
                assert!(
                    body.contains(link),
                    "Expected link {} not found in {}",
                    link,
                    file
                );
            }
        }
    }

    #[test]
    fn test_acyclic_dependencies() {
        let manifest_path = "../examples/dmn-manifest.yaml";
        let manifest = fs::read_to_string(manifest_path).unwrap();
        
        // Parse manifest and build dependency graph
        let mut edges: Vec<(String, String)> = Vec::new();
        
        let manifest: serde_yaml::Value = serde_yaml::from_str(&manifest).unwrap();
        if let Some(nodes_list) = manifest["nodes"].as_sequence() {
            for node in nodes_list {
                if let Some(deps) = node["dependencies"].as_sequence() {
                    let id = node["id"].as_str().unwrap().to_string();
                    for dep in deps {
                        edges.push((id.clone(), dep.as_str().unwrap().to_string()));
                    }
                }
            }
        }
        
        // Check for cycles using simple DFS
        fn has_cycle(node: &str, edges: &[(String, String)], visited: &mut Vec<String>) -> bool {
            if visited.contains(&node.to_string()) {
                return true;
            }
            visited.push(node.to_string());
            
            for (from, to) in edges {
                if from == node {
                    if has_cycle(to, edges, visited) {
                        return true;
                    }
                }
            }
            
            visited.pop();
            false
        }
        
        for (from, _) in &edges {
            let mut visited = Vec::new();
            assert!(
                !has_cycle(from, &edges, &mut visited),
                "Cycle detected in dependency graph"
            );
        }
    }
}