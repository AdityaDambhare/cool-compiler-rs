use scanner::Scanner;
use std::fs;
use std::io;
mod token;
mod scanner;
mod ast;
mod parser;

fn main() {
    //let file_path = "examples/hairyscary.cl" ;
    //let source = fs::read_to_string(file_path)
    //.expect("Should have been able to read the file");
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
    let tokens = scanner.scan_tokens();
    for tok in tokens.iter(){
        println!("{:?}",tok);
    }

    }
}


