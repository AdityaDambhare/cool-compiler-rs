use crate::ast::*;
use crate::token::*;
pub struct Parser{
    had_error : bool,
    tokens : Vec<Token>,
    current : usize
}
impl Parser{

pub fn new(tokens:Vec<Token>)->Parser{
    Parser{
        had_error:false,
        tokens,
        current:0
    }
}

pub fn parse(&mut self)->Expr{
    self.assignment()
}

fn error(&mut self,message:&str){
    self.had_error = true;
    eprintln!("Error: {}",message);
}

fn eof(&self)->bool{
    self.current >= self.tokens.len()
}

fn advance(&mut self)->&Token{
    if !self.eof(){
        self.current += 1;
    }
    &self.tokens[self.current - 1]
}

fn check(&self,token_type:TokenType)->bool{
    if self.eof(){
        false
    }
    else{
        self.tokens[self.current].clone().tokentype == token_type
    }
}

fn match_token(&mut self,token_type:TokenType)->bool{
    if self.check(token_type){
        self.advance();
        true
    }
    else{
        false
    }
}

fn assignment(&mut self)->Expr{
    let left = self.not();
    if self.match_token(TokenType::ASSIGN){
        let right = self.assignment();
        Expr::Assign(left,right)
    }
    else{
        left
    }
}

fn not(&mut self)->Expr{
    if self.match_token(TokenType::NOT){
        let expr = self.not();
        Expr::Not(expr)
    }
    else{
        self.comparison()
    }
}

fn comparison(&mut self)->Expr{
    let mut expr = self.term();
    while self.match_token(TokenType::LESSTHAN) || self.match_token(TokenType::LESSEQUAL) || self.match_token(TokenType::MORETHAN) || self.match_token(TokenType::MOREEQUAL){
        let op = self.tokens[self.current - 1].clone();
        let right = self.term();
        expr = Expr::Comparison(expr,op,right);
    }
    expr
}

fn term(&mut self)->Expr{
    let mut expr = self.factor();
    while self.match_token(TokenType::PLUS) || self.match_token(TokenType::MINUS){
        let op = self.tokens[self.current - 1].clone();
        let right = self.factor();
        expr = Expr::Arithmetic(expr,op,right);
    }
    expr
}

fn factor(&mut self)->Expr{
    let mut expr = self.isvoid();
    while(self.match_token(TokenType::STAR) || self.match_token(TokenType::SLASH)){
        let op = self.tokens[self.current - 1].clone();
        let right = self.isvoid();
        expr = Expr::Factor(expr,op,right);
    }
    expr
}

fn isvoid(&mut self)->Expr{
    if self.match_token(TokenType::ISVOID){
        let expr = self.isvoid();
        Expr::IsVoid(expr)
    }
    else{
        self.unary()
    }
}

fn unary(&mut self)->Expr{
    if self.match_token(TokenType::TILDA){
        let expr = self.unary();
        Expr::BitWiseNot(expr)
    }
    else{
        self.primary()
    }
}

fn primary(&mut self)->Expr{
    if self.match_token(TokenType::INTEGER){
        Expr::IntegerLiteral(self.tokens[self.current - 1].clone())
    }
    else if self.match_token(TokenType::STRING){
        Expr::StringLiteral(self.tokens[self.current - 1].clone())
    }
    else if self.match_token(TokenType::KEYTRUE){
        Expr::BoolLiteral(self.tokens[self.current - 1].clone())
    }
    else if self.match_token(TokenType::KEYFALSE){
        Expr::BoolLiteral(self.tokens[self.current - 1].clone())
    }
    else if self.match_token(TokenType::LEFTPAREN){
        let expr = self.assignment();
        if self.match_token(TokenType::RIGHTPAREN){
            expr
        }
        else{
            self.error("Expected ')' after expression");
            Expr::Error("".to_string())
        }
    }
    else{
        self.error("Expected expression");
        Expr::Error("".to_string())
    }
}

}