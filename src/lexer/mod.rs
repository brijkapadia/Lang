use super::parser::OperationType;
use crate::error as E;
use crate::reader::Reader;
use crate::tokens::TokenList;
use crate::tokens::TokenType;
mod error;
use error::LexerError;

#[derive(Debug)]
pub struct Tokenizer {
    src: Reader,
    pub tokens: TokenList,
}

impl Tokenizer {
    pub fn new(src: Reader) -> Tokenizer {
        let tokens = TokenList::new();
        Tokenizer { src, tokens }
    }

    pub fn tokenize(&mut self) -> E::Result<()> {
        while !self.src.eof() {
            //Check for math-like stuff (addition, subtraction, ect.)
            if self.tokenize_single_char()? {
                continue;
            } else if self.tokenize_double_char()? {
                continue;
            }
            //Create token for an number TODO: negative
            else if self.is_numeric()? {
                self.tokenize_number()?;
            } else if self.is_letter()? {
                self.tokenize_word()?;
            }
            //if char is a space, newline, ect. it will just get skipped
            else {
                self.src.consume();
            }
        }
        self.tokens.push_new_token(TokenType::EOF);
        Ok(())
    }

    //return true for while loop until a character is read or EOF
    fn until(&self, char: char) -> bool {
        !self.src.eof() && self.src.first().unwrap() != char
    }

    fn while_true(&self, expr: bool) -> bool {
        !self.src.eof() && expr
    }
    fn first(&self) -> E::Result<char> {
        self.src.first().ok_or(LexerError::UnexpectedEOF.into())
    }

    fn is_numeric(&self) -> E::Result<bool> {
        self.first().map(|c| c.is_numeric())
    }
    fn is_letter(&self) -> E::Result<bool> {
        let first_char = self.first()?;
        Ok(first_char.is_alphabetic() || first_char == '_')
    }
    fn is_alphanumeric(&self) -> E::Result<bool> {
        let is_numeric = self.is_numeric()?;
        let is_letter = self.is_letter()?;
        Ok(is_numeric || is_letter)
    }

    //TODO fix weather it is an int or float and the size
    fn tokenize_number(&mut self) -> E::Result<()> {
        let mut number_string = String::new();
        let mut token_type = TokenType::Int;
        while self.while_true(self.is_numeric()?) {
            number_string.push(self.src.consume());
            let is_float = self.first()?;
            if is_float == '.' {
                token_type = TokenType::Float;
                number_string.push(self.src.consume());
            }
        }

        if matches!(token_type, TokenType::Int) {
            let number: i32 = number_string.parse()?;
            self.tokens.push_new_integer(number, token_type);
        } else if matches!(token_type, TokenType::Float) {
            let number: f32 = number_string.parse()?;
            self.tokens.push_new_float(number, token_type);
        }

        Ok(())
    }

    fn tokenize_string(&mut self) -> E::Result<()> {
        self.src.expect('\"')?;
        let mut string = String::new();
        while self.until('\"') {
            string.push(self.src.consume());
            if self.src.eof() {
                return Err(LexerError::TokenExpectedBeforEOF('\"'.to_string()).into());
            }
        }
        self.src.expect('\"')?;
        self.tokens.push_new_string(string, TokenType::String);
        Ok(())
    }
    fn tokenize_char(&mut self) -> E::Result<()> {
        self.src.expect('\'')?;
        let char = self.src.consume();
        self.src.expect('\'')?;
        self.tokens.push_new_character(char, TokenType::Char);
        Ok(())
    }

    fn tokenize_single_char(&mut self) -> E::Result<bool> {
        match self.first()? {
            '(' => self.tokens.push_new_token(TokenType::OpenParen),
            ')' => self.tokens.push_new_token(TokenType::CloseParen),
            '{' => self.tokens.push_new_token(TokenType::OpenCurl),
            '}' => self.tokens.push_new_token(TokenType::CloseCurl),
            '[' => self.tokens.push_new_token(TokenType::OpenSquare),
            ']' => self.tokens.push_new_token(TokenType::CloseSquare),
            ';' => self.tokens.push_new_token(TokenType::EOL),
            '%' => self
                .tokens
                .push_new_operation(OperationType::Mod, TokenType::BinaryOp),
            '\'' => self.tokenize_char()?,
            '\"' => self.tokenize_string()?,
            ',' => self.tokens.push_new_token(TokenType::Comma),
            _ => return Ok(false),
        }
        self.src.consume();
        Ok(true)
    }
    fn tokenize_single_part(&mut self) -> E::Result<bool> {
        match self.first()? {
            '+' => self
                .tokens
                .push_new_operation(OperationType::Add, TokenType::BinaryOp),
            '-' => self
                .tokens
                .push_new_operation(OperationType::Subtract, TokenType::BinaryOp),
            '*' => self
                .tokens
                .push_new_operation(OperationType::Multiply, TokenType::BinaryOp),
            '/' => self
                .tokens
                .push_new_operation(OperationType::Divide, TokenType::BinaryOp),
            '=' => self.tokens.push_new_token(TokenType::Assign),
            '>' => self
                .tokens
                .push_new_operation(OperationType::Greater, TokenType::BinaryComp),
            '<' => self
                .tokens
                .push_new_operation(OperationType::Less, TokenType::BinaryComp),
            _ => return Ok(false),
        }
        self.src.consume();
        Ok(true)
    }

    fn tokenize_double_char(&mut self) -> E::Result<bool> {
        let first_char = self.first()?;
        let mut second_char = '\0';
        let mut str = String::new();
        match self.src.peak(1) {
            Option::None => {
                return self.tokenize_single_part();
            }
            Some(c) => second_char = c,
        }
        str.push(first_char);
        str.push(second_char);
        match str.as_str() {
            "+=" => self.tokens.push_new_token(TokenType::Increment),
            "-=" => self.tokens.push_new_token(TokenType::Decrement),
            "**" => self
                .tokens
                .push_new_operation(OperationType::Power, TokenType::BinaryOp),
            ">=" => self
                .tokens
                .push_new_operation(OperationType::GreaterEq, TokenType::BinaryComp),
            "<=" => self
                .tokens
                .push_new_operation(OperationType::LessEq, TokenType::BinaryComp),
            "!=" => self
                .tokens
                .push_new_operation(OperationType::NotEq, TokenType::BinaryComp),
            "==" => self
                .tokens
                .push_new_operation(OperationType::Eq, TokenType::BinaryComp),
            _ => return self.tokenize_single_part(),
        }
        self.src.consume();
        self.src.consume();
        Ok(true)
    }
    fn tokenize_word(&mut self) -> E::Result<()> {
        let mut word = String::new();
        while self.is_alphanumeric()? {
            word.push(self.src.consume());
        }
        match word.as_str() {
            "let" => self.tokens.push_new_token(TokenType::Let),
            "if" => self.tokens.push_new_token(TokenType::If),
            "elif" => self.tokens.push_new_token(TokenType::Elif),
            "else" => self.tokens.push_new_token(TokenType::Else),
            "while" => self.tokens.push_new_token(TokenType::While),
            "and" => self
                .tokens
                .push_new_operation(OperationType::And, TokenType::BinaryComp),
            "or" => self
                .tokens
                .push_new_operation(OperationType::Or, TokenType::BinaryComp),
            "not" => self
                .tokens
                .push_new_operation(OperationType::Not, TokenType::BinaryComp),
            "fn" => self.tokens.push_new_token(TokenType::Function),
            "True" => self.tokens.push_new_boolean(true, TokenType::Bool),
            "False" => self.tokens.push_new_boolean(false, TokenType::Bool),
            _ => self.tokenize_identifier(word),
        }
        Ok(())
    }

    fn tokenize_identifier(&mut self, word: String) {
        self.tokens.push_new_identifier(word, TokenType::Identifier);
    }
}
