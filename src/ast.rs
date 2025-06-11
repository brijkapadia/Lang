use std::rc::Rc;

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
    VarDec(VarDec),
    VarUpdate(VarUpdate),

    Scope(Scope),

    If(If),
    Else(Else),
    Elif(Elif),
    While(While),

    Expr(Expr),    
}

impl Stmt{
    pub fn new_var_dec(name: Rc<String>, value: Option<Expr>) -> Self{
        Self::VarDec(VarDec::new(name, value))
    }
}

#[derive(Debug)]
pub struct VarDec {
    pub name: Rc<String>,
    pub value: Option<Expr>
}

impl VarDec{
    pub fn new(name: Rc<String>, value: Option<Expr>) -> Self{
        Self{name, value: value}
    }
}

#[derive(Debug)]
pub struct VarUpdate {
    pub name: String,
    pub value: Expr
}

impl VarUpdate{
    pub fn new(name: String, value: Expr) -> Self{
        Self{name, value}
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