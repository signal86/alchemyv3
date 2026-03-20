use super::syntax::*;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub enum Issue {
    Error(Error),
    Warn(String),
}

pub struct Context {
    definitions: HashMap<String, ComponentDefinition>,
    instances: HashMap<Option<String>, ComponentInstance>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
            instances: HashMap::new(),
        }
    }

    pub fn define_component(&mut self, inst: ComponentDefinition) {
        self.definitions.insert(inst.identifier.clone(), inst);
    }

    pub fn initialize_component(&mut self, inst: ComponentInstance) {
        self.instances.insert(inst.identifier.clone(), inst);
    }
}

fn valid_definition(
    context: &Context,
    definition: &ComponentDefinition,
) -> Result<Vec<Issue>, Vec<Issue>> {
    let mut warnings: Vec<Issue> = Vec::new();
    let mut errors: Vec<Issue> = Vec::new();

    if context.definitions.contains_key(&definition.identifier) {
        warnings.push(Issue::Warn(format!(
            "redefinition of {}",
            &definition.identifier
        )));
    }

    if let Some(n) = &definition.default_var {
        match definition.vars.binary_search(&n) {
            Ok(_) => {}
            Err(_) => errors.push(Issue::Error(Error::new(
                ErrorKind::Other,
                "default variable is not a defined variable in the specified component",
            ))),
        }
    }

    if errors.len() > 0 {
        return Err(errors);
    }

    Ok(warnings)
}

fn meta_components_analysis(ast: &AST, context: &mut Context) -> Option<Vec<(u128, Issue)>> {
    let mut issues: Vec<(u128, Issue)> = Vec::new();

    // Meta components
    let mut component_scope = false;
    for node in ast.nodes.iter() {
        if !component_scope && !matches!(node.node, ASTNodeT::Meta(Meta::Components)) {
            continue;
        } else if component_scope && matches!(node.node, ASTNodeT::Meta(_)) {
            break;
        }
        component_scope = true;

        match &node.node {
            ASTNodeT::ComponentDefinition(inst) => {
                // defined_components.push(inst.clone());
                match valid_definition(&context, &inst) {
                    Ok(n) => {
                        if n.len() > 0 {
                            for issue in n.into_iter() {
                                issues.push((node.line, issue));
                            }
                        }
                    }
                    Err(e) => {}
                }
                // context.define_component(inst.clone());
            }
            ASTNodeT::Meta(_) => (),
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

// !! Potential optimization: When going to search for components, store locations of other metas and
// everything else
fn meta_view_analysis(ast: &AST, context: &mut Context) -> Option<Vec<(u128, Issue)>> {
    let mut issues: Vec<(u128, Issue)> = Vec::new();

    let mut component_scope = false;
    for node in ast.nodes.iter() {
        if !component_scope && !matches!(node.node, ASTNodeT::Meta(Meta::View)) {
            continue;
        } else if component_scope && matches!(node.node, ASTNodeT::Meta(_)) {
            break;
        }
        component_scope = true;

        match &node.node {
            ASTNodeT::ComponentInstance(inst) => {
                context.initialize_component(inst.clone());
            }
            ASTNodeT::ComponentDefinition(inst) => {
                context.define_component(inst.clone());
                issues.push((
                    node.line,
                    Issue::Warn(
                        "component definitions are not recommended inside 'meta views'".to_string(),
                    ),
                ));
            }
            ASTNodeT::Assignment(_) => {
                // check existence
            }
            ASTNodeT::Meta(_) => (),
        }
    }

    if !issues.is_empty() {
        return Some(issues);
    }

    None
}

pub fn semantic_analysis(ast: &AST) -> Option<Vec<(u128, Issue)>> {
    let mut issues: Vec<(u128, Issue)> = Vec::new();
    let mut context: Context = Context::new();

    match meta_components_analysis(ast, &mut context) {
        None => {}
        Some(i) => issues.extend(i.into_iter()),
    }

    match meta_view_analysis(ast, &mut context) {
        None => {}
        Some(i) => issues.extend(i.into_iter()),
    }

    if !issues.is_empty() {
        return Some(issues);
    }

    None
}
