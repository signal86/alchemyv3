use super::syntax::*;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub enum Issue {
    Error(Error),
    Warn(String),
}

#[derive(Debug)]
pub struct Context {
    definitions: HashMap<String, ComponentDefinition>,
    instance_ids: HashMap<usize, ComponentInstance>,
    // ComponentInstance.identifier is key if it exists, otherwise the id will
    // not be entered into this map
    inverse_instance_ids: HashMap<String, usize>,
    // (count - 1) will act as last_initialized
    // last_initialized: Option<usize>,
    count: usize,
}

impl Context {
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
            instance_ids: HashMap::new(),
            inverse_instance_ids: HashMap::new(),
            // last_initialized: None,
            count: 1, // lazy anti-underflow, bc unsigned
        }
    }

    pub fn define_component(&mut self, inst: ComponentDefinition) {
        self.definitions.insert(inst.identifier.clone(), inst);
    }

    pub fn initialize_component(&mut self, inst: ComponentInstance) {
        // self.instances.insert(inst.identifier.clone(), inst);
        match &inst.identifier {
            None => {
                self.instance_ids.insert(self.count, inst);
                // self.last_initialized = Some(self.count);
                self.count += 1;
            }
            Some(s) => {
                self.inverse_instance_ids.insert(s.clone(), self.count);
                self.instance_ids.insert(self.count, inst);
                // self.last_initialized = Some(self.count);
                self.count += 1;
            }
        }
    }

    pub fn is_defined(&self, search: &str) -> bool {
        self.definitions.contains_key(&(search.to_string()))
    }

    pub fn get_definition(&self, search: &str) -> Result<&ComponentDefinition, Error> {
        // println!(
        //     "Instance IDs: {:#?}\nInverse Instance IDs: {:#?}\nDefinitions: {:#?}",
        //     self.instance_ids, self.inverse_instance_ids, self.definitions
        // );
        // self.definitions
        //     .get(&search.to_string())
        //     .ok_or_else(|| Error::new(ErrorKind::Other, "No component definition exists"))
        match self.get_instance(&search) {
            Ok(inst) => {
                return self
                    .definitions
                    .get(&inst.component)
                    .ok_or_else(|| Error::new(ErrorKind::Other, "No component definition exists"));
            }
            Err(e) => return Err(e),
            // true => {
            //     let a = self.inverse_instance_ids.get(&search.to_string()).unwrap();
            //     let instance = self.instance_ids.get(a).unwrap();
            //     return self
            //         .definitions
            //         .get(&instance.component)
            //         .ok_or_else(|| Error::new(ErrorKind::Other, "No component definition exists"));
            // }
            // false => Err(Error::new(
            //     ErrorKind::Other,
            //     "No component definition exists",
            // )),
        }
        // match self.is_defined(&search) {
        //     true => self
        //         .definitions
        //         .get(&search.to_string())
        //         .ok_or_else(|| Error::new(ErrorKind::Other, "No component definition exists")),
        //     false => Err(Error::new(
        //         ErrorKind::Other,
        //         "No component definition exists",
        //     )),
        // }
    }

    pub fn is_initialized(&self, search: &str) -> bool {
        self.inverse_instance_ids
            .contains_key(&(search.to_string()))
    }

    pub fn get_instance(&self, search: &str) -> Result<&ComponentInstance, Error> {
        // let e = Error::new(ErrorKind::Other, "No component definition exists");
        let id = self
            .inverse_instance_ids
            .get(&(search.to_string()))
            .ok_or_else(|| Error::new(ErrorKind::Other, "No component is initialized"))?;
        self.instance_ids
            .get(id)
            .ok_or_else(|| Error::new(ErrorKind::Other, "No component is initialized"))
    }

    pub fn get_instance_mut(&mut self, search: &str) -> Result<&mut ComponentInstance, Error> {
        // let e = Error::new(ErrorKind::Other, "No component definition exists");
        let id = self
            .inverse_instance_ids
            .get(&(search.to_string()))
            .ok_or_else(|| Error::new(ErrorKind::Other, "No component is initialized"))?;
        self.instance_ids
            .get_mut(id)
            .ok_or_else(|| Error::new(ErrorKind::Other, "No component is initialized"))
    }

    pub fn last_component(&self) -> Result<&ComponentInstance, Error> {
        self.instance_ids
            .get(&(self.count - 1))
            .ok_or(Error::new(ErrorKind::Other, "No component is initialized"))
    }

    pub fn last_component_mut(&mut self) -> Result<&mut ComponentInstance, Error> {
        self.instance_ids
            .get_mut(&(self.count - 1))
            .ok_or(Error::new(ErrorKind::Other, "No component is initialized"))
    }
}

fn valid_definition(
    context: &Context,
    definition: &ComponentDefinition,
) -> Result<Vec<Issue>, Vec<Issue>> {
    let mut warnings: Vec<Issue> = Vec::new();
    let mut errors: Vec<Issue> = Vec::new();

    // if context.definitions.contains_key(&definition.identifier) {
    if context.is_defined(definition.identifier.as_str()) {
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
                match valid_definition(&context, &inst) {
                    Ok(n) => {
                        if n.len() > 0 {
                            for issue in n.into_iter() {
                                issues.push((node.line, issue));
                            }
                        }
                    }
                    Err(e) => {
                        for issue in e.into_iter() {
                            issues.push((node.line, issue));
                        }
                    }
                }
                context.define_component(inst.clone());
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
                match valid_definition(&context, &inst) {
                    Ok(n) => {
                        if n.len() > 0 {
                            for issue in n.into_iter() {
                                issues.push((node.line, issue));
                            }
                        }
                    }
                    Err(e) => {
                        for issue in e.into_iter() {
                            issues.push((node.line, issue));
                        }
                    }
                }
                context.define_component(inst.clone());

                issues.push((
                    node.line,
                    Issue::Warn(
                        "component definitions are not recommended inside 'meta views'".to_string(),
                    ),
                ));
            }
            ASTNodeT::Assignment(assgn) => {
                // check existence
                if !context.is_initialized(assgn.component_identifier.as_str()) {
                    issues.push((
                        node.line,
                        Issue::Error(Error::new(
                            ErrorKind::Other,
                            "Component does not exist".to_string(),
                        )),
                    ));
                } else {
                    // check specified field exists
                    // println!("line: {}", node.line);
                    let def = context
                        .get_definition(assgn.component_identifier.as_str())
                        .unwrap();
                    match def.vars.contains(&assgn.field) {
                        true => (),
                        false => issues.push((
                            node.line,
                            Issue::Error(Error::new(
                                ErrorKind::Other,
                                "Field specified by assignment is not part of the component"
                                    .to_string(),
                            )),
                        )),
                    }
                }
            }
            ASTNodeT::Meta(_) => (),
        }
    }

    if !issues.is_empty() {
        return Some(issues);
    }

    None
}

pub fn semantic_analysis(ast: &AST, context: &mut Context) -> Option<Vec<(u128, Issue)>> {
    let mut issues: Vec<(u128, Issue)> = Vec::new();
    // let mut context: Context = Context::new();

    match meta_components_analysis(ast, context) {
        None => {}
        Some(i) => issues.extend(i.into_iter()),
    }

    match meta_view_analysis(ast, context) {
        None => {}
        Some(i) => issues.extend(i.into_iter()),
    }

    if !issues.is_empty() {
        return Some(issues);
    }

    None
}
