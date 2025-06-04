use crate::error as E;
use std::io::{BufRead, BufReader, Read};
use std::fs;
use std::error::Error;
use std::io;

#[derive(Debug)]
pub struct Reader {
    code: Vec<char>,
}

impl Reader {
    pub fn new(path_to_code: &str) -> io::Result<Self> {
        let code_string = fs::read_to_string(path_to_code)?;
        let code_vec: Vec<char> = code_string.chars().collect();
        Ok(Self {
            code: code_vec,
        })
    }
    pub fn first(&self) -> Option<char>{
        self.code.first().copied()
    }
    pub fn consume(&mut self) -> char {
        self.code.remove(0)
    }

    pub fn expect(&mut self, expect_char: char) -> E::Result<char>{
        if let Some(current_char) = self.first() {
            if current_char == expect_char{
                return Ok(self.consume());
            }
            return Err(
                format!("Expected a \"{}\" but found \"{}\" instead",expect_char, current_char).into()
            );
        }
        return Err(
            format!("Expected a \"{}\" but did not find any remaining characters",expect_char).into()
        );
        
    }
    // pub fn expect_before_eof(&mut self, expect_char: char) -> Result<(), String>{
    //     if let Some(_) = self.first() {
    //         return Ok(())
    //     }
    //     return Err(
    //         format!("Expected a \"{}\" but did not find any remaining characters",expect_char)
    //     );
    // }
    
    pub fn peak(&mut self, index: usize) -> Option<char>{
        return self.code.get(index).copied();
    }

    pub fn eof(&self) -> bool{
        self.code.len() == 0
    }
}