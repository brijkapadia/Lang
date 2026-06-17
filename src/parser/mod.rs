use crate::ast;
use crate::error::Result as R;
use crate::tokens;
use crate::tokens::Token;
use crate::tokens::TokenData;
use crate::tokens::TokenList;
use crate::tokens::TokenType;
mod expr;
mod expr_types;
mod identifier;
use crate::ast::VariableData;
pub use expr_types::Expr;
pub use expr_types::OperationType;
pub use identifier::Variables;
use std::cell::Cell;
use std::rc::Rc;

pub struct Parser {
    tokens: TokenList,
    pub program_scope: ast::Scope,
}

impl Parser {
    pub fn new(tokens: TokenList) -> Parser {
        Parser {
            tokens,
            program_scope: ast::Scope::new(),
        }
    }
    fn current_token_guarantee(&self) -> R<&Token> {
        self.tokens.first().ok_or("Unexpected EOF".into())
    }
    fn current_token_type_guarantee(&mut self) -> R<&TokenType> {
        self.tokens.first_type().ok_or("Unexpected EOF".into())
    }
    fn consume(&mut self) -> Token {
        self.tokens.pop_token()
    }
    fn expect(&mut self, token_type: TokenType) -> R<()> {
        let current_token_type = self.consume().token_type;
        match current_token_type {
            _ => Ok(()),
            _ => Err(format!(
                "Expected {:?} but found {:?}",
                token_type, current_token_type
            )
            .into()),
        }
    }
    fn not_eof(&self) -> bool {
        !self.tokens.eof()
    }

    pub fn produce_ast(&mut self) -> R<()> {
        while self.not_eof() {
            let mut variables = Variables::new(None);
            let parse = self.parse_stmt(&mut variables)?;
            self.program_scope.push(parse);
        }
        Ok(())
    }

    fn parse_expr(&mut self, variables: &Variables) -> R<expr_types::Expr> {
        let mut expr_parser = expr::ExprParser::new(variables, &mut self.tokens);
        expr_parser.parse_expr(0.0)
    }

    fn parse_stmt(&mut self, variables: &mut Variables) -> R<ast::Stmt> {
        let token_type = self.current_token_type_guarantee()?;
        match token_type {
            TokenType::Let => self.parse_variable_declaration(variables),
            TokenType::Identifier => self.parse_identifier(variables),

            TokenType::OpenCurl => self.parse_scope(variables).map(ast::Stmt::Scope),

            TokenType::If => self.parse_if(variables),
            TokenType::Elif => self.parse_elif(variables),
            TokenType::Else => self.parse_else(variables),

            TokenType::While => self.parse_while(variables),
            _ => self.parse_expr(variables).map(ast::Stmt::Expr),
        }
    }
    fn parse_variable_declaration(&mut self, variables: &mut Variables) -> R<ast::Stmt> {
        self.expect(TokenType::Let)?;
        let var: Token = self.consume();

        let var_name = match var.token_data {
            tokens::TokenData::Identifier(name) => name,
            _ => return Err("Expected an Identifier after let".into()),
        };
        if let TokenType::Assign = self.current_token_guarantee()?.token_type {
            self.expect(TokenType::Assign)?;
            let data = Rc::new(VariableData {
                variable_bytes: 4,
                stack_position: Cell::new(0),
            });
            variables.add_variable_by_name(var_name.into(), Rc::clone(&data));
            let statement: ast::Stmt = ast::Stmt::new_var_dec(data, self.parse_expr(variables)?);
            self.expect(TokenType::EOL)?;
            Ok(statement)
        } else {
            Err("Never found \"=\" after variable declaration".into())
        }
    }

    fn parse_identifier(&mut self, variables: &Variables) -> R<ast::Stmt> {
        if let TokenData::Identifier(name) = self.consume().token_data {
            let Some(data) = variables.get_variable_by_name(&name) else {
                return Err(
                    "Tried to write to a variable that does not exist yet. Add \"let\" before"
                        .into(),
                );
            };

            let data = Rc::clone(data);

            self.expect(TokenType::Assign)?;
            let value: Expr = self.parse_expr(variables)?;
            self.expect(TokenType::EOL)?;

            Ok(ast::Stmt::VariableUpdate(ast::VariableUpdate {
                data,
                value,
            }))
        } else {
            panic!()
        }
    }

    fn parse_scope(&mut self, variables: &mut Variables) -> R<ast::Scope> {
        self.consume(); //first {

        let mut scope = ast::Scope::new();

        while !matches!(
            self.current_token_type_guarantee()?,
            tokens::TokenType::CloseCurl
        ) {
            scope.push(self.parse_stmt(variables)?);
        }
        self.consume();
        Ok(scope)
    }

    fn parse_if(&mut self, variables: &mut Variables) -> R<ast::Stmt> {
        self.consume(); //consumes if

        let condition = self.parse_expr(variables)?;

        let if_scope = self.parse_scope(variables)?;
        Ok(ast::Stmt::If(ast::If::new(condition, if_scope)))
    }
    fn parse_elif(&mut self, variables: &mut Variables) -> R<ast::Stmt> {
        self.consume(); //consumes if

        let condition = self.parse_expr(variables)?;
        let elif_scope = self.parse_scope(variables)?;
        Ok(ast::Stmt::Elif(ast::Elif::new(condition, elif_scope)))
    }
    fn parse_while(&mut self, variables: &mut Variables) -> R<ast::Stmt> {
        self.consume(); //consumes while

        let condition = self.parse_expr(variables)?;
        let while_scope = self.parse_scope(variables)?;
        let x = ast::Stmt::While(ast::While::new(condition, while_scope));
        Ok(x)
    }
    fn parse_else(&mut self, variables: &mut Variables) -> R<ast::Stmt> {
        self.consume(); //consumes else

        let else_scope = self.parse_scope(variables)?;
        Ok(ast::Stmt::Else(ast::Else::new(else_scope)))
    }
}
