use crate::ast::VariableData as Variable;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Variables<'a> {
    variables_in_scope: HashMap<Box<str>, Rc<Variable>>,
    variables_out_scope: Option<&'a Variables<'a>>,
}

impl<'a> Variables<'a> {
    pub fn new(variables_out_scope: Option<&'a Self>) -> Self {
        Self {
            variables_in_scope: HashMap::new(),
            variables_out_scope,
        }
    }
    pub fn get_variable_by_name(&self, name: &str) -> Option<&Rc<Variable>> {
        match self.variables_in_scope.get(name) {
            Some(var) => Some(var),
            Option::None => match self.variables_out_scope {
                Some(variables) => variables.get_variable_by_name(name),
                Option::None => None,
            },
        }
    }
    pub fn add_variable_by_name(&mut self, name: Box<str>, variable: Rc<Variable>) {
        self.variables_in_scope.insert(name, variable);
    }
}
