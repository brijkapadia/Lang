use crate::ast;
use crate::tokens;
use crate::tokens::TokenList;
use crate::tokens::TokenType;
use crate::tokens::Token;

enum OpType{
    Expr,
    Add,
    Mul,
    Pow,
    Paren
}
pub struct Parser{
    tokens: TokenList, 
    pub program_scope: ast::Scope,
    variables: Vec<String>
}


impl Parser{
    pub fn new(tokens: TokenList) -> Parser{
        Parser{tokens, program_scope: ast::Scope::new(), variables: Vec::new()}
    }
    fn current_token(&self) -> Option<&Token>{
        self.tokens.first()
    }
    fn current_token_type(&mut self) -> Option<&TokenType>{
        self.tokens.first_type()
    }
    fn consume(&mut self) -> Token{
        self.tokens.pop_token()
    }
    fn not_eof(&self) -> bool{
        !self.tokens.eof()
    }

    pub fn produce_ast(&mut self){
        while self.not_eof() {
            let parse = self.parse_stmt();
            self.program_scope.push(parse);
        }
    }

    fn parse_stmt(&mut self) -> ast::Stmt {
        if let Some(token_type) = self.current_token_type(){
            match token_type{
            TokenType::Let => return self.parse_variable_declaration(),
            TokenType::Identifier => return self.parse_identifier(),

            TokenType::OpenCurl => return ast::Stmt::Scope(self.parse_scope()),

            TokenType::If => return self.parse_if(),
            TokenType::Elif => return self.parse_elif(),
            TokenType::Else => return self.parse_else(),

            TokenType::While => return self.parse_while(),

            _ => return ast::Stmt::Expr(self.parse_expr())
        }
        }
        panic!()
        
    }
    fn parse_scope(&mut self) -> ast::Scope{
        self.consume(); //first {

        let mut scope = ast::Scope::new();

        while !matches!(self.current_token_type(),tokens::TokenType::CloseCurl){
            let parse = self.parse_stmt();
            scope.push(parse);
        }

        self.consume(); //consumes close curl
        
        return scope;
    }

    fn parse_if(&mut self) -> ast::Stmt{
        self.consume(); //consumes if

        let condition = self.parse_expr();
        let if_scope = self.parse_scope();
        ast::Stmt::If(ast::If::new(condition,if_scope))
    }
    fn parse_elif(&mut self) -> ast::Stmt{
        self.consume(); //consumes if

        let condition = self.parse_expr();
        let elif_scope = self.parse_scope();
        ast::Stmt::Elif(ast::Elif::new(condition,elif_scope))
    }
    fn parse_while(&mut self) -> ast::Stmt{
        self.consume(); //consumes while

        let condition = self.parse_expr();
        let while_scope = self.parse_scope();
        ast::Stmt::While(ast::While::new(condition,while_scope))
    }
    fn parse_else(&mut self) -> ast::Stmt{
        self.consume(); //consumes else

        let else_scope = self.parse_scope();
        ast::Stmt::Else(ast::Else::new(else_scope))
    }

    fn parse_identifier(&mut self) -> ast::Stmt{
        let identifier = self.consume();
        if !self.variables.contains(&identifier.value){
            panic!("Tried to write to a variable that does not exist yet. Add \"let\" before")
        } 
        else if !(self.consume().value == "="){
            panic!("Expected \"=\" after variable")
        }
        let value = self.parse_expr();
        if !(self.consume().value == ";"){
            panic!("Did not find \";\" at end of line")
        } // gets rid of ;

        ast::Stmt::VarUpdate(ast::VarUpdate::new(identifier.value, value))
    }
    fn parse_variable_declaration(&mut self) -> ast::Stmt{
        if !matches!(self.consume().token_type, TokenType::Let){
            panic!("Tried to declare variable in parser but never found \"let\"")
        }
        let var = self.consume();
        self.variables.push(var.value.clone());
        if self.current_token_value() == ";"{
            self.consume();
            ast::Stmt::new_var_dec(var.value, None)
        } else if self.consume().value == "="{
            let statement = ast::Stmt::new_var_dec(var.value, Some(self.parse_expr()));
            self.consume(); //get rid of last semicolon
            statement
        }else{
            panic!("Never found \"=\" after variable declaration")
        }
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
    fn parse_expr(&mut self) -> ast::Expr{
        self.order_of_op(OpType::Expr)
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

