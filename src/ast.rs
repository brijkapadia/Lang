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
    pub fn new_var_dec(name: String, value: Option<Expr>) -> Self{
        Self::VarDec(VarDec::new(name, value))
    }
}

pub struct VarDec {
    pub name: String,
    pub value: Option<Expr>
}

impl VarDec{
    pub fn new(name: String, value: Option<Expr>) -> Self{
        Self{name, value: value}
    }
}

pub struct VarUpdate {
    pub name: String,
    pub value: Expr
}

impl VarUpdate{
    pub fn new(name: String, value: Expr) -> Self{
        Self{name, value}
    }
}

pub struct If {
    pub condition: Expr,
    pub scope: Scope
}

impl If{
    pub fn new(condition: Expr, scope: Scope) -> Self{
        Self {condition, scope}
    }
}

pub struct Elif {
    pub condition: Expr,
    pub scope: Scope
}

impl Elif{
    pub fn new(condition: Expr, scope: Scope) -> Self{
        Self {condition, scope}
    }
}

pub struct Else {
    pub scope: Scope
}

impl Else{
    pub fn new(scope: Scope) -> Self{
        Self {scope}
    }
}

pub struct While {
    pub condition: Expr,
    pub scope: Scope
}

impl While{
    pub fn new(condition: Expr, scope: Scope) -> Self{
        Self {condition, scope}
    }
}

pub enum Expr {
    BinaryExpr(BinaryExpr), // like +, -, *, /
    IntLiteral(IntLiteral),
    FloatLiteral(FloatLiteral),
    BoolLiteral(BoolLiteral),
    VarLiteral(VarLiteral)
}

impl Expr{
    pub fn new_binary_expr(left: Expr,right: Expr, op: OperationType) -> Expr{
        Expr::BinaryExpr(BinaryExpr::new(left, right, op))
    }
    pub fn new_int_literal(value: i64) -> Expr{
        Expr::IntLiteral(IntLiteral{value})
    }
    pub fn new_float_literal(value: f64) -> Expr{
        Expr::FloatLiteral(FloatLiteral{value})
    }
    pub fn new_bool_literal(value: bool) -> Expr{
        Expr::BoolLiteral(BoolLiteral{value})
    }
    pub fn new_var_literal(name: String) -> Expr{
        Expr::VarLiteral(VarLiteral{name})
    }
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

pub struct IntLiteral {
    pub value: i64,
}
pub struct FloatLiteral {
    pub value: f64,
}

pub struct BoolLiteral{
    pub value: bool
}

pub struct VarLiteral{
    pub name: String
}