mod ast;
//mod compiler;
mod error;
mod lexer;
mod parser;
mod reader;
mod tokens;
use error as E;
use std::env;
use std::thread;
use std::time::Duration;

fn get_file_name() -> String {
    env::args().nth(1).unwrap_or("main.txt".to_string())
}

fn main() -> E::Result<()> {
    let file_name = get_file_name();
    let src = reader::Reader::new(&file_name)?;
    dbg!(&src);
    thread::sleep(Duration::from_secs(2));
    let mut tokenizer = lexer::Tokenizer::new(src);

    tokenizer.tokenize()?;
    let tokens = tokenizer.tokens;
    dbg!(&tokens);
    thread::sleep(Duration::from_secs(2));
    let mut parser = parser::Parser::new(tokens);
    parser.produce_ast()?;
    let program = parser.program_scope;
    dbg!(program);
    Ok(())
    //let mut compiler = compiler::Compiler::new("test");
    //compiler.compile(program);

    //Ok(())
}
