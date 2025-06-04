use crate::ast;
use crate::tokens;
use crate::tokens::TokenList;
use crate::tokens::TokenType;
use crate::tokens::Token;

enum OpType{
    Expr,
    
    Or,
    And,
    Not,

    Equality,

    Comparison,

    AddSubtract,
    
    MultiplyDivide,

    Pow,
    
    Paren
}

struct ExprParser<'a>{
    tokens: &'a TokenList
}

impl<'a> ExprParser<'a>{
    fn new(tokens: &'a TokenList) -> Self{
        Self {tokens}
    }
    fn parse_expr(&mut self) -> ast::Expr{
        self.order_of_op(OpType::Expr)
    }

    fn parse_binary_op_from_type(&mut self, op_type: OpType,token_types: [TokenType]) -> ast::Expr{
        let mut left = self.order_of_op(op_type);
        while token_types.contains(self.tokens.first_type()){
            let op = self.tokens.consume();
            let right = self.order_of_op(op_type);
            left = ast::Expr::new_binary_expr(left,right,op);
        }
        left
    }

    fn order_of_op(&mut self,op: OpType) -> ast::Expr{
        match op{
            OpType::Expr => return self.parse_add(),
            OpType::Add => return self.parse_mul(),
            OpType::Mul=> return self.parse_pow(),
            OpType::Pow => return self.parse_paren(),
            OpType::Paren=> return self.parse_primary(),
        }
    }

    fn parse_add(&mut self) -> ast::Expr{
        let mut left = self.order_of_op(OpType::Add);
        while self.current_token_value() == "+" || self.current_token_value() == "-"{
            let op = self.consume().value;
            let right = self.order_of_op(OpType::Add);
            left = ast::Expr::new_binary_expr(left,right,op);
        }
        left
    }


    fn parse_pow(&mut self) -> ast::Expr{
        self.order_of_op(OpType::Pow)
    }
    fn parse_paren(&mut self) -> ast::Expr{
        let expr_p: ast::Expr;
        if self.current_token_value() == "("{
            self.consume();
            expr_p = self.parse_expr();
            if self.current_token_value() != ")"{
                panic!("Expected a closing paren");
            } else{
                self.consume();
                expr_p
            }
        } else{
            self.order_of_op(OpType::Paren)
        }
    }

    fn parse_mul(&mut self) -> ast::Expr{
        let mut left = self.order_of_op(OpType::Mul);
        while self.current_token_value() == "*" || self.current_token_value() == "/"{
            let op = self.consume().value;
            let right = self.order_of_op(OpType::Mul);
            left = ast::BinaryExpr::new(left,right,op);
        }
        left
    }
    fn parse_primary(&mut self) -> ast::Expr{
        let token_type = self.current_token_type();
        match token_type{
            TokenType::Int => return ast::IntLiteral::new(self.consume().value),
            TokenType::Float => return ast::FloatLiteral::new(self.consume().value),
            TokenType::Identifier => return ast::VarLiteral::new(self.consume().value),
            _ => panic!("Found something else that is not a primary expression {:?}", token_type)
        }
    }
}