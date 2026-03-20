use super::syntax::*;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub enum Issue {
    Error(Error),
    Warn(String),
}

fn meta_components_analysis(ast: &AST) -> Option<Vec<(u128, Issue)>> {
    let mut issues: Vec<(u128, Issue)> = Vec::new();

    // Meta components
    let mut component_scope = false;
    let mut defined_components: Vec<ComponentDefinition> = Vec::new();
    for node in ast.nodes.iter() {
        if !component_scope && !matches!(node.node, ASTNodeT::Meta(Meta::Components)) {
            continue;
        }
        if matches!(node.node, ASTNodeT::Meta(Meta::View)) {
            break;
        }
        component_scope = true;

        match &node.node {
            ASTNodeT::ComponentDefinition(inst) => {
                defined_components.push(inst.clone());
            }
            ASTNodeT::Meta(Meta::Components) => (),
            _ => issues.push((
                node.line,
                Issue::Error(Error::new(
                    ErrorKind::Other,
                    format!("unexpected: {:?} -- 'meta components' only supports the definition of components", node.node),
                )),
            )),
        }
    }

    if !issues.is_empty() {
        return Some(issues);
    }

    None
}

pub fn semantic_analysis(ast: &AST) -> Option<Vec<(u128, Issue)>> {
    let mut issues: Vec<(u128, Issue)> = Vec::new();

    match meta_components_analysis(ast) {
        None => {}
        Some(i) => issues.extend(i.into_iter()),
    }

    if !issues.is_empty() {
        return Some(issues);
    }

    None
}
