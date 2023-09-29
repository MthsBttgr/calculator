use std::env;

use crate::eval::Eval;
use crate::lexer::Lexer;
use crate::parser::Parser;

mod eval;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let calculation = args[1].clone();

    let tokens = Lexer::new(calculation).lex();

    let mut parser = Parser::new(tokens.clone());
    let ast = parser.parse();

    println!("{:?}", tokens);
    println!("=================================================");
    println!("{:#?}", ast);
    println!("=================================================");
    println!("{:#?}", ast.eval());
}
