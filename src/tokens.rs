use crate::ast::OperationType;

#[derive(Debug)]
pub enum TokenType{
    //Numbers
    Int,
    Float,
    Bool,
    
    //Variable or Functions let [identifier] =(assign) {Expr}
    Let,
    If,
    While,
    Elif,
    Else,

    Function,

    Identifier,

    Assign,
    Increment,
    Decrement,
    
    Comma,

    BinaryOp,
    BinaryComp,

    OpenParen,
    CloseParen,
    OpenCurl,
    CloseCurl,
    OpenSquare,
    CloseSquare,

    String,
    Char,

    //End of line (;)
    EOL,
    //End of File
    EOF
}

#[derive(Debug)]
pub enum TokenData{
    Identifier(String),
    Integer(i32),
    Float(f32),
    Character(char),
    String(String),
    Boolean(bool),
    Operation(OperationType),
    None
}

#[derive(Debug)]
pub struct Token{
    pub token_data: TokenData,
    pub token_type: TokenType
}

impl Token{
    fn new(token_data: TokenData, token_type: TokenType) -> Self{
        Self {token_data, token_type}
    }
    pub fn new_identifier(identifier: String, token_type: TokenType) -> Self {
        Self::new(TokenData::Identifier(identifier), token_type)
    }

    pub fn new_integer(value: i32, token_type: TokenType) -> Self {
        Self::new(TokenData::Integer(value), token_type)
    }

    pub fn new_float(value: f32, token_type: TokenType) -> Self {
        Self::new(TokenData::Float(value), token_type)
    }

    pub fn new_character(value: char, token_type: TokenType) -> Self {
        Self::new(TokenData::Character(value), token_type)
    }

    pub fn new_string(value: String, token_type: TokenType) -> Self {
        Self::new(TokenData::String(value), token_type)
    }

    pub fn new_boolean(value: bool, token_type: TokenType) -> Self {
        Self::new(TokenData::Boolean(value), token_type)
    }

    pub fn new_operation(op: OperationType, token_type: TokenType) -> Self {
        Self::new(TokenData::Operation(op), token_type)
    }
    pub fn new_token(token_type: TokenType) -> Self{
        Self::new(TokenData::None,token_type)
    }

}

#[derive(Debug)]
pub struct TokenList{
    pub token_list: Vec<Token>
}

impl TokenList{
    pub fn new() -> Self{
        Self{token_list: Vec::new()}
    }
    pub fn push_token(&mut self, token: Token){
        self.token_list.push(token);
    }
    pub fn pop_token(&mut self) -> Token{
        self.token_list.remove(0)
    }

    pub fn push_new_identifier(&mut self, identifier: String, token_type: TokenType){
        self.push_token(Token::new_identifier(identifier, token_type));
    }

    pub fn push_new_integer(&mut self, value: i32, token_type: TokenType){
        self.push_token(Token::new_integer(value, token_type));
    }

    pub fn push_new_float(&mut self, value: f32, token_type: TokenType){
        self.push_token(Token::new_float(value, token_type));
    }

    pub fn push_new_character(&mut self, value: char, token_type: TokenType){
        self.push_token(Token::new_character(value, token_type));
    }

    pub fn push_new_string(&mut self, value: String, token_type: TokenType){
        self.push_token(Token::new_string(value, token_type));
    }

    pub fn push_new_boolean(&mut self, value: bool, token_type: TokenType){
        self.push_token(Token::new_boolean(value, token_type));
    }

    pub fn push_new_operation(&mut self, op: OperationType, token_type: TokenType){
        self.push_token(Token::new_operation(op, token_type));
    }
    pub fn push_new_token(&mut self, token_type: TokenType){
        self.push_token(Token::new_token(token_type));
    }


    pub fn first(&self) -> Option<&Token>{
        self.token_list.first()
    }
    pub fn first_type(&self) -> Option<&TokenType>{
        self.first().map(|token| &token.token_type)
    }
    
    pub fn eof(&self) -> bool{
        self.first().map_or(true,|token| matches!(token.token_type,TokenType::EOF))
    }
}
