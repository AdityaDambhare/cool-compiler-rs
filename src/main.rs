use scanner::Scanner;
use parser::Parser;

use std::io;
mod token;
mod scanner;
mod parser;
mod ast;
mod astprinter;

fn main() {
    runprompt();
}

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

    let mut  p =  Parser::new(tokens);

    let prog = match p.parse_program(){
        Ok(prog) => prog,
        Err(e) => {
            println!("{}",e);
            continue;
        }
    };
    println!("{}",astprinter::AstPrinter::print_program(&prog));
}
}



