mod reader;
mod tokens;
mod lexer;

mod parser;
mod ast;
// mod compiler;
mod error;
use std::env;
// use std::fs;
use error as E;

fn main() -> E::Result<()>{
    //env::set_var("RUST_BACKTRACE", "1");  
    let file_name = env::args().nth(1).ok_or::<E::Error>("No file name was provided".into())?;
    let src = reader::Reader::new(&file_name)?;

    let mut tokenizer = lexer::Tokenizer::new(src);

    tokenizer.tokenize()?;
    let tokens = tokenizer.tokens;
    
    //return Ok(())
    
    let mut parser = parser::Parser::new(tokens);
    parser.produce_ast();
    let program = parser.program_scope;

    println!("{:?}",program);


    return Ok(());

    // // let mut compiler = compiler::Compiler::new("test");
    // // compiler.compile(program);
}
