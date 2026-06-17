use crate::error as E;
use std::fs;
use std::io;

#[derive(Debug)]
pub struct Reader {
    code: Vec<char>,
}

impl Reader {
    pub fn new(path_to_code: &str) -> io::Result<Self> {
        let code_string = fs::read_to_string(path_to_code)?;
        let code_vec: Vec<char> = code_string.chars().collect();
        Ok(Self { code: code_vec })
    }
    pub fn first(&self) -> Option<char> {
        self.code.first().copied()
    }
    pub fn consume(&mut self) -> char {
        self.code.remove(0)
    }

    pub fn expect(&mut self, expect_char: char) -> E::Result<char> {
        if let Some(current_char) = self.first() {
            if current_char == expect_char {
                return Ok(self.consume());
            }
            return Err(format!(
                "Expected a \"{}\" but found \"{}\" instead",
                expect_char, current_char
            )
            .into());
        }
        Err(format!(
            "Expected a \"{}\" but did not find any remaining characters",
            expect_char
        )
        .into())
    }
    pub fn peak(&mut self, index: usize) -> Option<char> {
        self.code.get(index).copied()
    }

    pub fn eof(&self) -> bool {
        self.code.is_empty()
    }
}
