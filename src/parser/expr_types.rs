use crate::ast::VariableData;
use std::rc::Rc;

#[derive(Debug)]
pub enum Expr {
    BinaryExpr(BinaryExpr), // like +, -, *, /
    Atom(Atom),
}

impl Expr{
    pub fn new_binary_expr(left: Expr,right: Expr, op: OperationType) -> Expr{
        Expr::BinaryExpr(BinaryExpr::new(left, right, op))
    }
    pub fn new_int_literal(value: i32) -> Expr{
        Expr::Atom(Atom::IntLiteral(value))
    }
    pub fn new_float_literal(value: f32) -> Expr{
        Expr::Atom(Atom::FloatLiteral(value))
    }
    pub fn new_bool_literal(value: bool) -> Expr{
        Expr::Atom(Atom::BoolLiteral(value))
    }
    pub fn new_var_literal(data: Rc<VariableData>) -> Expr{
        Expr::Atom(Atom::VariableLiteral(data))
    }
    pub fn new_char_literal(value: char) -> Expr{
        Expr::Atom(Atom::CharLiteral(value))
    }
    pub fn new_string_literal(value: String) -> Expr{
        Expr::Atom(Atom::StringLiteral(value.into()))
    }
}

#[derive(Debug)]
pub enum Atom{
    IntLiteral(i32),
    FloatLiteral(f32),
    BoolLiteral(bool),
    VariableLiteral(Rc<VariableData>),
    CharLiteral(char),
    StringLiteral(Box<str>)
}

#[derive(Debug)]
pub enum OperationType{
    Add,
    Subtract,
    
    Multiply,
    Divide,
    Power,

    Mod,

    And,
    Or,
    Not,

    Greater,
    Less,
    GreaterEq,
    LessEq,
    Eq,
    NotEq
}

impl OperationType {
    pub fn get_binding_power(&self) -> (f32,f32){
        match self{
            Self::Greater | Self::GreaterEq | Self::Less | Self::LessEq | Self::Eq => (1., 1.),
            Self::Mod => (2.,2.),
            Self::Add | Self::Subtract => (2.,2.),
            Self::Multiply | Self::Divide => (3.,3.),
            _ => todo!()
        }
    }
}
#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub op: OperationType,
}

impl BinaryExpr{
    pub fn new(left: Expr, right: Expr, op: OperationType)-> Self{
        Self{
            left: Box::new(left),
            right: Box::new(right),
            op
        }
    }
}