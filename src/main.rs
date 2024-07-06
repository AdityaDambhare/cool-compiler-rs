use scanner::Scanner;
use parser::Parser;

use std::io;
mod token;
mod scanner;
mod parser;
mod ast;
mod astprinter;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if(args.len() > 2){
        println!("Usage: jcc [script]");
        std::process::exit(64);
    }
    else if args.len() == 2{
        runFile();
    }
    else{
        runprompt();
    }
}


fn runFile(){
    let filename = std::env::args().nth(1).unwrap();
    let source = match std::fs::read_to_string(&filename){
        Ok(source) => source,
        Err(e) => {
            println!("Error reading file: {}",e);
            std::process::exit(74);
        }
    };
    let mut scanner = Scanner::new(source);
    let tokens = match scanner.scan_tokens(){
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("{}",e);
            std::process::exit(65);
        }
    };

    for tok in tokens.iter(){
        println!("{:?}",tok);
    }

    let mut  p =  Parser::new(tokens);

    let prog = match p.parse_program(){
        Ok(prog) => prog,
        Err(e) => {
            eprintln!("{}",e);
            std::process::exit(65);
        }
    };
    println!("{}",astprinter::AstPrinter::print_program(&prog));
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



