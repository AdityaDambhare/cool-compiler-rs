use scanner::Scanner;
use std::fs;
mod token;
mod scanner;

fn main() {
    let file_path = "examples/hairyscary.cl" ;
    let source = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for tok in tokens.iter(){
        println!("{:?}",tok);
    }
}


