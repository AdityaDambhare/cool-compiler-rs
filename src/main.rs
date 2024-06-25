use scanner::Scanner;
use parser::Parser;
use ast::Expr;
use std::io;
mod token;
mod scanner;
mod parser;
mod ast;

fn runprompt(){
    loop
    {
    let mut source = String::new();
    println!("\nEnter the source code: ");
    match io::stdin().read_line(&mut source){
        Ok(_) => (),
        Err(e) => {
            println!("Error reading from stdin: {}",e);
            continue;
        }
    };
    let mut scanner = Scanner::new(source);
    let tokens = match scanner.scan_tokens(){
        Ok(tokens) => tokens,
        Err(e) => {
            println!("{}",e);
            continue;
        }
    };

    for tok in tokens.iter(){
        println!("{:?}",tok);
    }
    let mut  p = Parser::new(tokens);
    let expr = p.parse();
    println!("{:?}",expr);
}
}
fn main() {
    runprompt();
}


