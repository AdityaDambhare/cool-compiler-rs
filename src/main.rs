use scanner::Scanner;

mod token;
mod scanner;
fn main() {
    let mut scanner = Scanner::new("--.@~isvoid*/+-\n<=!=<<-not".to_string());
    let tokens = scanner.scan_tokens();
    for tok in tokens.iter(){
        println!("{:?}",tok);
    }
}


