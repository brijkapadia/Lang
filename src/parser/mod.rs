use crate::ast;
use crate::tokens;
use crate::tokens::TokenData;
use crate::tokens::TokenList;
use crate::tokens::TokenType;
use crate::tokens::Token;
use crate::error::Result as R;
mod expr;
mod expr_types;
//mod identifier;

pub use expr_types::Expr;
pub use expr_types::OperationType;

use std::rc::Rc;

pub struct Parser{
    tokens: TokenList, 
    pub program_scope: ast::Scope,
}


impl Parser{
    pub fn new(tokens: TokenList) -> Parser{
        Parser{tokens, program_scope: ast::Scope::new()}
    }
    fn current_token_guarantee(&self) -> R<&Token>{
        self.tokens.first().ok_or("Unexpected EOF".into())
    }
    fn current_token_type_guarantee(&mut self) -> R<&TokenType>{
        self.tokens.first_type().ok_or("Unexpected EOF".into())
    }
    fn consume(&mut self) -> Token{
        self.tokens.pop_token()
    }
    fn expect(&mut self, token_type: TokenType) -> R<()>{
        let current_token_type = self.consume().token_type;
        match current_token_type{
            token_type =>Ok(()),
            _ => Err(format!("Expected {:?} but found {:?}",token_type,current_token_type).into())
        }
    }
    fn not_eof(&self) -> bool{
        !self.tokens.eof()
    }

    pub fn produce_ast(&mut self) -> R<()>{
        while self.not_eof() {
            let parse = self.parse_stmt()?;
            self.program_scope.push(parse);
        }
        Ok(())
    }

    fn parse_expr(&mut self) -> R<expr_types::Expr>{
        let mut expr_parser = expr::ExprParser::new(&mut self.tokens);
        expr_parser.parse_expr(0.0)
    }

    fn parse_stmt(&mut self) -> R<ast::Stmt>{
        let token_type = self.current_token_type_guarantee()?;
        match token_type{
            TokenType::Let => self.parse_variable_declaration(),
            TokenType::Identifier => self.parse_identifier(),

            TokenType::OpenCurl => self.parse_scope().map(|x| ast::Stmt::Scope(x)),

            TokenType::If => self.parse_if(),
            TokenType::Elif => self.parse_elif(),
            TokenType::Else => self.parse_else(),

            TokenType::While => self.parse_while(),
            _ => self.parse_expr().map(|x| ast::Stmt::Expr(x))
        }
        
    }
    fn parse_variable_declaration(&mut self) -> R<ast::Stmt>{
        self.expect(TokenType::Let)?;

        let var = self.consume();

        let var_name = Rc::new(
            match var.token_data {
            tokens::TokenData::Identifier(name) => name,
            _ => return Err("Expected an Identifier after let".into())
            
        });
        //self.identifiers.push(Identfiers::new_var(Rc::clone(&var_name)));

        if matches!(self.current_token_guarantee()?.token_type,TokenType::EOL){
            self.consume();
            Ok(ast::Stmt::new_var_dec(Rc::clone(&var_name), None))
        }
        else if matches!(self.current_token_guarantee()?.token_type,TokenType::Assign){
            self.consume();
            let statement = ast::Stmt::new_var_dec(Rc::clone(&var_name), Some(self.parse_expr()?));
            self.consume(); //get rid of last semicolon
            Ok(statement)
        }else{
            Err("Never found \"=\" after variable declaration".into())
        }
    }

    fn parse_identifier(&mut self) -> R<ast::Stmt>{
        if let TokenData::Identifier(name) = self.consume().token_data{
            // if !self.variables.contains(&identifier.value){
            //     panic!("Tried to write to a variable that does not exist yet. Add \"let\" before")
            // } 
            if !matches!(self.consume().token_type,TokenType::Assign){
                return Err("Expected \"=\" after variable".into());
            }
            let value: Expr = self.parse_expr()?;
            if !matches!(self.consume().token_type,TokenType::EOL){
                return Err("Did not find \";\" at end of line".into());
            } // gets rid of ;

            Ok(ast::Stmt::VarUpdate(ast::VarUpdate::new(name, value)))
        }else{
            panic!()
        }
    }

    fn parse_scope(&mut self) -> R<ast::Scope>{
        self.consume(); //first {

        let mut scope = ast::Scope::new();

        while !matches!(self.current_token_type_guarantee()?,tokens::TokenType::CloseCurl){
            scope.push(self.parse_stmt()?);
        }
        self.consume();        
       Ok(scope)
    }

    fn parse_if(&mut self) -> R<ast::Stmt>{
        self.consume(); //consumes if

        let condition = self.parse_expr()?;
        
        let if_scope = self.parse_scope()?;
        Ok(ast::Stmt::If(ast::If::new(condition,if_scope)))
    }
    fn parse_elif(&mut self) -> R<ast::Stmt>{
        self.consume(); //consumes if

        let condition = self.parse_expr()?;
        let elif_scope = self.parse_scope()?;
        Ok(ast::Stmt::Elif(ast::Elif::new(condition,elif_scope)))
    }
    fn parse_while(&mut self) -> R<ast::Stmt>{
        self.consume(); //consumes while

        let condition = self.parse_expr()?;
        let while_scope = self.parse_scope()?;
        let x = ast::Stmt::While(ast::While::new(condition,while_scope));
        Ok(x)
    }
    fn parse_else(&mut self) -> R<ast::Stmt>{
        self.consume(); //consumes else

        let else_scope = self.parse_scope()?;
        Ok(ast::Stmt::Else(ast::Else::new(else_scope)))
    }

    
    
}

