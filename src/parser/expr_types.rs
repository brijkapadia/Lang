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
    pub fn new_var_literal(name: String) -> Expr{
        Expr::Atom(Atom::VarLiteral(name))
    }
}

#[derive(Debug)]
enum Atom{
    IntLiteral(i32),
    FloatLiteral(f32),
    BoolLiteral(bool),
    VarLiteral(String)
}

#[derive(Debug)]
pub enum OperationType{
    Add,
    Subtract,
    
    Multiply,
    Divide,
    Power,

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
            Self::Add | Self::Subtract => (1.,1.),
            Self::Multiply | Self::Divide => (2.,2.),
            Self::Greater | Self::GreaterEq | Self::Less | Self::LessEq | Self::Eq => (3., 3.),
            _ => panic!()
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