use super::expr_types::Expr;
use super::expr_types::BinaryExpr;
use crate::tokens;
use crate::tokens::TokenData;
use crate::tokens::TokenList;
use crate::tokens::TokenType;
use crate::tokens::Token;
//use crate::parser::Identfiers;

use crate::error::Result as R;

struct ExprParser<'a>{
    tokens: &'a mut TokenList,
    identifiers: &'a Identfiers
}

impl<'a> ExprParser<'a>{
    fn new(tokens: &'a mut TokenList,identifiers: &'a Identfiers) -> Self{
        Self {tokens, identifiers}
    }

    fn parse_expr(&mut self, prev_binding_power: f32) -> R<Expr>{
        let left_token = self.tokens.pop_token();
        let mut left_expression = match left_token.token_data {
            TokenData::Identifier(identifier) => self.parse_identifier(identifier),
            TokenData::Integer(int) => Ok(Expr::new_int_literal(int)),
            TokenData::Float(float) => Ok(Expr::new_float_literal(float)),
            TokenData::Boolean(bool) => Ok(Expr::new_bool_literal(bool)),
            TokenData::Character(char) => panic!("chars not available yet"),
            TokenData::String(str) => panic!("str not available yet"),
            TokenData::Operation(op) => return Err(format!("Found operation {:?} at the start of an expression",op).into()),
            TokenData::None => return Err("Found token with no data".into())
        };

        loop{
            let next_token =  self.tokens.first().ok_or("EOF before finished parsing expression")?;
            if matches!(next_token.token_type,TokenType::EOL){
                break Ok(left_expression)
            }
            if let TokenData::Operation(op) = &next_token.token_data{
                let binding_power = op.get_binding_power();
                if prev_binding_power > binding_power.0{
                    break Ok(left_expression)
                } else{
                    let right_expression = self.parse_expr(binding_power.1)?;
                    if let  TokenData::Operation(op_type )= self.tokens.pop_token().token_data{
                        left_expression = Expr::BinaryExpr(BinaryExpr::new(left_expression,right_expression,op_type));
                    }
                }
            } else{
                break Err(format!("Found non operation token in expression: {:?}",next_token).into())
            }
        }
    }

    fn parse_identifier(&self, identifier: String) -> R<Expr>{
        if matches!(self.tokens.first().ok_or("unexpected eof")?.token_type,TokenType::OpenParen){
            self.parse_function()
        }
        self.parse_variable(identifier)
    }

    fn parse_function(&self){
        panic!("Not implimented")
    }

    fn parse_variable(&self, var_name: String) -> R<Expr>{
        Ok(Expr::new_var_literal(var_name))
    }
}