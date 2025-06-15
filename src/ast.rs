use std::{cell::Cell, rc::Rc};

use super::parser::Expr;

#[derive(Debug)]
pub struct Scope {
    pub body: Vec<Stmt>,
}

impl Scope{
    pub fn new() -> Self{
        let body: Vec<Stmt> = Vec::new();
        Self{body}
    }
    pub fn push(&mut self,stmt: Stmt){
        self.body.push(stmt);
    }
}

#[derive(Debug)]
pub enum Stmt {
    VariableDeclaration(VariableDeclaration),
    VariableUpdate(VariableUpdate),

    Scope(Scope),

    If(If),
    Else(Else),
    Elif(Elif),
    While(While),

    Expr(Expr),    
}

impl Stmt{
    pub fn new_var_dec(data: Rc<VariableData>, value: Expr) -> Self{
        Self::VariableDeclaration(VariableDeclaration{data, value})
    }
}

#[derive(Debug)]
pub struct VariableData{
    pub variable_bytes: usize,
    pub stack_position: Cell<usize>
}

#[derive(Debug)]
pub struct VariableDeclaration{
    pub data: Rc<VariableData>,
    pub value: Expr
}

#[derive(Debug)]
pub struct VariableUpdate {
    pub data: Rc<VariableData>,
    pub value: Expr
}

impl VariableUpdate{
    pub fn new(data: Rc<VariableData>, value: Expr) -> Self{
        Self{data, value}
    }
}

#[derive(Debug)]
pub struct If {
    pub condition: Expr,
    pub scope: Scope
}

impl If{
    pub fn new(condition: Expr, scope: Scope) -> Self{
        Self {condition, scope}
    }
}

#[derive(Debug)]
pub struct Elif {
    pub condition: Expr,
    pub scope: Scope
}


impl Elif{
    pub fn new(condition: Expr, scope: Scope) -> Self{
        Self {condition, scope}
    }
}
#[derive(Debug)]
pub struct Else {
    pub scope: Scope
}

impl Else{
    pub fn new(scope: Scope) -> Self{
        Self {scope}
    }
}

#[derive(Debug)]
pub struct While {
    pub condition: Expr,
    pub scope: Scope
}

impl While{
    pub fn new(condition: Expr, scope: Scope) -> Self{
        Self {condition, scope}
    }
}