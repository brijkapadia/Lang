use super::expr_types::Expr;
use super::expr_types::BinaryExpr;
use crate::tokens;
use crate::tokens::TokenData;
use crate::tokens::TokenList;
use crate::tokens::TokenType;
use crate::tokens::Token;
//use crate::parser::Identfiers;

use crate::error::Result as R;

pub struct ExprParser<'a>{
    tokens: &'a mut TokenList,
    //identifiers: &'a Identfiers
}

impl<'a> ExprParser<'a>{
    pub fn new(tokens: &'a mut TokenList) -> Self{
        Self {tokens}
    }

    pub fn parse_expr(&mut self, prev_binding_power: f32) -> R<Expr>{
        let left_token = self.tokens.pop_token();
        if let TokenType::OpenParen = left_token.token_type{
            self.parse_expr(0.0)?;

        }
        let mut left_expression = 
        match left_token.token_type{
            TokenType::OpenParen => {
                let result = self.parse_expr(0.0)?;
                self.tokens.pop_token();
                result
            }
            _ => match left_token.token_data {
                TokenData::Identifier(identifier) => self.parse_identifier(identifier)?,
                TokenData::Integer(int) => Expr::new_int_literal(int),
                TokenData::Float(float) => Expr::new_float_literal(float),
                TokenData::Boolean(bool) => Expr::new_bool_literal(bool),
                TokenData::Character(char) => Expr::new_char_literal(char),
                TokenData::String(str) => Expr::new_string_literal(str),
                TokenData::Operation(op) => return Err(format!("Found operation {:?} at the start of an expression",op).into()),
                TokenData::None => return Err("Found token with no data".into())
            }
        };

        loop{
            let next_token =  self.tokens.first_guarantee()?;
            match &next_token.token_data{
            TokenData::Operation(op) =>{
                let (left_binding_power,right_binding_power) = op.get_binding_power();
                if prev_binding_power > left_binding_power{
                    return Ok(left_expression)
                } 
                let TokenData::Operation(op_type ) = self.tokens.pop_token().token_data else{
                    unreachable!("could not consume op type");
                };
                let right_expression = self.parse_expr(right_binding_power)?;
                left_expression = Expr::BinaryExpr(BinaryExpr::new(left_expression,right_expression,op_type));
            } 
            _ => return Ok(left_expression)
            }
        }
    }

    fn parse_identifier(&self, identifier: String) -> R<Expr>{
        if matches!(self.tokens.first_guarantee()?.token_type,TokenType::OpenParen){
            self.parse_function()
        }
        self.parse_variable(identifier)
    }

    fn parse_function(&self){
        todo!("Not implimented")
    }

    fn parse_variable(&self, var_name: String) -> R<Expr>{
        Ok(Expr::new_var_literal(var_name))
    }
}