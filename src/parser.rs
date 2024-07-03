use crate::ast::*;
use crate::token::*;
pub struct Parser{
    had_error : bool,
    panic_mode : bool,
    tokens : Vec<Token>,
    current : usize
}
impl Parser{

pub fn new(tokens:Vec<Token>)->Parser{
    Parser{
        had_error:false,
        panic_mode:false,
        tokens,
        current:0
    }
}



fn error(&mut self,message:&str,line:usize){
    if self.panic_mode{
        return;
    }
    self.had_error = true;
    eprintln!("Error at line {}: {}",line,message);
}

fn eof(&self)->bool{
    self.current >= self.tokens.len()-1
}
fn previous(&mut self)->&Token{
    if self.current == 0 {
        return &self.tokens[0];
    }
    &self.tokens[self.current - 1]
}
fn peek(&self)->&Token{
    &self.tokens[self.current]
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
        self.peek().clone().tokentype == token_type
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

fn consume(&mut self,token_type:TokenType,message:&str)->Token{
    if self.check(token_type){
        self.advance().clone()
    }
    else{
       
        self.error(message,self.peek().line);
        self.panic_mode = true;
        self.synchronize();
        Token::new(0," ".to_string(),TokenType::ERROR,None)
    }
}

pub fn synchronize(&mut self){
    self.advance();
    while !self.eof(){
        match self.peek().tokentype{
            TokenType::SEMICOLON => {
                self.advance();
                return;
            }
            TokenType::KEYCLASS => {
                return;
            }
            _ => {self.advance();}
        }
    }
}

pub fn parse_program(&mut self)->Result<Program,&str>{
        let mut classes:Vec<Class> = vec![];
        while !self.eof() {
            let class = self.parse_class();
            classes.push(class);
            if self.had_error {break;}
            self.consume(TokenType::SEMICOLON, "Expected ; after class definition");
        }
        if self.had_error {Err("error in parsing")} else {Ok(Program::new(classes))}
}

fn parse_class(&mut self)->Class{
    self.consume(TokenType::KEYCLASS, "Expected class definition");
    let name = self.consume(TokenType::IDENTIFIER,"Expected class name");
    if name.lexeme.chars().nth(0).unwrap().is_ascii_lowercase(){
        self.error(format!("error at \"class {}\". Class name should start with an uppercase letter.",
        name.lexeme.clone())
        .as_str(),
        name.line);
    }
    let inherits = match self.match_token(TokenType::KEYINHERITS){
        true =>{ let name = self.consume(TokenType::IDENTIFIER,"Expected superclass name");
        if name.lexeme.chars().nth(0).unwrap().is_ascii_lowercase(){
            self.error(format!("error at \"inherits {}\". Class name should start with an uppercase letter.",
            name.lexeme.clone())
            .as_str(),
            name.line);
        }
        Some(name)
        },
        false => None
    };
    self.consume(TokenType::LEFTBRACE,"Expected { after class declaration");
    let mut features : Vec<Feature> = vec![];
    while !self.check(TokenType::RIGHTBRACE) && !self.eof(){
        features.push(self.parse_feature());
        if self.had_error {break;}
        self.consume(TokenType::SEMICOLON,"Expect ; after method or attribute definition");
    }
    self.consume(TokenType::RIGHTBRACE, "Expect } after class definition");
    Class::new(name, inherits, features)
}

fn parse_feature(&mut self)->Feature{
    let id = self.consume(TokenType::IDENTIFIER, "Expect identifier");
    if self.match_token(TokenType::LEFTPAREN){
        self.parse_method(id)
    }
    else if self.match_token(TokenType::COLON){
        self.parse_attribute(id)
    }
    else{
        self.error("Illegal Feature syntax",self.peek().line);
        self.synchronize();
        Feature::new_attribute(id, self.previous().clone(), None)
    }
    
}

fn parse_attribute(&mut self,id:Token)->Feature{
    let type_ = self.consume(TokenType::IDENTIFIER, "Expect type name");
    let expr = match self.match_token(TokenType::ASSIGN){
        true => Some(self.expression()),
        false => None
    };
    Feature::new_attribute(id, type_, expr)
}

fn parse_method(&mut self,id:Token)->Feature{
    let mut formals : Vec<Formal> = vec![];
    while !self.check(TokenType::RIGHTPAREN){
        formals.push(self.parse_formal());
    }
    self.consume(TokenType::RIGHTPAREN, "Expect ) after method parameters");
    self.consume(TokenType::COLON,"Expect : before return type");
    let type_ = self.consume(TokenType::IDENTIFIER, "Expect return type from method");
    self.consume(TokenType::LEFTBRACE, "Expect { after method declaration");
    let expr = self.expression();
    self.consume(TokenType::RIGHTBRACE, "Expect } after method body");
    Feature::new_method(id, type_, formals, expr)
}


fn parse_formal(&mut self)->Formal{
    let id = self.consume(TokenType::IDENTIFIER, "expect identifier");
    self.consume(TokenType::COLON,"Expect colon after identifier");
    let type_ = self.consume(TokenType::IDENTIFIER,"Expect type name");
    self.match_token(TokenType::COMMA);//eating up the comma if it exists
    Formal::new(id,type_)
}

pub fn parse_expression(&mut self )->Result<Expr,&str>{
    let expr = self.expression();
   match self.had_error{
         true => Err("Error in parsing"),
         false => Ok(expr)
   }
}
fn expression(&mut self)->Expr{
    self.assignment()
}

fn if_expr(&mut self)->Expr{
    let expr = self.expression();
    self.consume(TokenType::KEYTHEN,"Expected 'then' after if");
    let then_expr = self.expression();
    self.consume(TokenType::KEYELSE, "expect else after then");
    let else_expr = self.expression();
    self.consume(TokenType::KEYFI,"Expected 'fi' after else");
    Expr::IF_EXPR(expr,then_expr,else_expr)
}

fn block(&mut self)->Expr{
    let mut exprs = Vec::new();
    while !self.eof() && !self.check(TokenType::RIGHTBRACE){
        exprs.push(self.expression());
        self.consume(TokenType::SEMICOLON,"Use ; to seperate expressions in block");
    }
    if exprs.is_empty() {self.error("Empty block",self.peek().line);}
    self.consume(TokenType::RIGHTBRACE, "Expected '}' after block");
    Expr::BLOCK_EXPR(exprs)
}

fn while_expr(&mut self)->Expr{
    let condition = self.expression();
    self.consume(TokenType::KEYLOOP,"Expected 'loop' after condition");
    let body = self.expression();
    self.consume(TokenType::KEYPOOL,"Expected 'pool' after body");
    Expr::WHILE_EXPR(condition,body)
}

fn let_expr(&mut self)->Expr{
    let mut declarations : Vec<Expr> = vec![];
    while !self.check(TokenType::KEYIN){
        let id = self.consume(TokenType::IDENTIFIER, "Expect identifier in Let expression");
        self.consume(TokenType::COLON,"Expect ':' after identifier in Let expression");
        let type_ = self.consume(TokenType::IDENTIFIER,"Expect type after identifier in Let expression");
        let expr = match self.match_token(TokenType::ASSIGN){
            true => Some(self.expression()),
            false => None
        };
        declarations.push(Expr::Declaration(id,type_,expr));
        if !self.match_token(TokenType::COMMA){
            break;
        }
    }
    self.consume(TokenType::KEYIN, "Expect 'in' after Let declarations");
    let body = self.expression();
    Expr::LET_EXPR(declarations, body)
}

fn case(&mut self)->Expr{
    let expr = self.expression();
    self.consume(TokenType::KEYOF, "Expect 'of' after case expression");
    let mut branches = Vec::new();
    while self.match_token(TokenType::IDENTIFIER) {
        let id = self.previous().clone();
        self.consume(TokenType::COLON,"Expected ':' after case branch");
        let type_ = self.consume(TokenType::IDENTIFIER,"Expected type after case branch");
        self.consume(TokenType::RARROW,"Expected '=>' after case branch type");
        let body = self.expression();
        self.consume(TokenType::SEMICOLON, "expect ; after case branch body");
        branches.push(Expr::Branch{id,type_,expr:Box::new(body)});
    }
    if branches.is_empty() {self.error("Empty case expression",self.peek().line);}
    self.consume(TokenType::KEYESAC,"Expected 'esac' after case expression");
    Expr::CASE_EXPR(expr,branches)
}

fn assignment(&mut self)->Expr{
    let left = self.not();

    if self.match_token(TokenType::ASSIGN){
        match left{
            Expr::ID{id:token} => {
                let right = self.assignment();
                Expr::Assign(Expr::ID(token),right)
            }
            _ => {
                self.error("Invalid assignment target",self.peek().line);
                Expr::Error  
            }
        
        }
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
    while self.match_token(TokenType::LESSTHAN) || self.match_token(TokenType::LESSEQUAL) || self.match_token(TokenType::MORETHAN) || self.match_token(TokenType::MOREEQUAL)|| self.match_token(TokenType::EQUALITY){
        let op = self.previous().clone();
        let right = self.not();
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
    while self.match_token(TokenType::STAR) || self.match_token(TokenType::SLASH) {
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
        let expr = self.expression();
        Expr::BitWiseNot(expr)
    }
   
    else{
        self.dispatch(None)
    }
}

fn dispatch(&mut self,expr:Option<Expr>)->Expr{
    let mut  expr = match expr { Some(e) => e ,None =>self.primary()};
    if(self.check(TokenType::LEFTPAREN)){
        if self.previous().tokentype != TokenType::IDENTIFIER{
            self.error("expression not a method name so it cannot be called", self.peek().line);
        }
        self.consume(TokenType::LEFTPAREN, "");
        expr = self.patch_dispatch(expr,None,None);
    }
    let mut type_present = false;
    let mut type_ = match self.match_token(TokenType::AT){
        true =>{ type_present=true;Some(self.consume(TokenType::IDENTIFIER, "Expect type name after @"))},
        false => None
    };
    if (type_present && !self.check(TokenType::DOT)){
        self.consume(TokenType::DOT,"Expect method call after @ expression");
    }
    while(self.match_token(TokenType::DOT)){
        if (!type_present){type_ = None;} 
        let id = self.consume(TokenType::IDENTIFIER, "Expect method name for dispatch");
        self.consume(TokenType::LEFTPAREN,"cannot access attribute . add '()' after method name in case you want to call it");
        expr = self.patch_dispatch(expr,type_.clone(),Some(id));
        type_present = false;
    }
    if self.check(TokenType::AT){
        return self.dispatch(Some(expr));
    }
    if self.match_token(TokenType::LEFTPAREN){
        let prev = self.previous().line;
        self.error("Dispatch works on methods only . maybe you forgot adding  a '.'", prev);
        self.consume(TokenType::RIGHTPAREN,"");
    } 
    expr
}

fn patch_dispatch(&mut self,expr:Expr,type_:Option<Token>,id:Option<Token>)->Expr{
    let mut arguments : Vec<Expr> = vec![];
    while(!self.check(TokenType::RIGHTPAREN)){
        arguments.push(self.assignment());
        if !self.match_token(TokenType::COMMA){break;}
    }
    self.consume(TokenType::RIGHTPAREN,"Expect ')' after call");
    Expr::Dispatch(type_, expr, id, arguments)
}

fn primary(&mut self)->Expr{
    if self.match_token(TokenType::IDENTIFIER){
        Expr::ID(self.tokens[self.current - 1].clone())
    }
    else if self.match_token(TokenType::INTEGER){
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
    else if self.match_token(TokenType::KEYNEW){
        let type_ = self.consume(TokenType::IDENTIFIER,"Expected type after new");
        Expr::New(type_)
    }
    else if self.match_token(TokenType::KEYDELETE){
        let expr = self.expression();
        Expr::Delete(expr)
    }
    else if self.match_token(TokenType::LEFTBRACE){
        self.block()
    }
    else if self.match_token(TokenType::LEFTPAREN){
        let expr = self.expression();
        self.consume(TokenType::RIGHTPAREN,"Expected ')' after expression");
        Expr::Grouping(expr)   
    }
    else if self.match_token(TokenType::KEYIF) {
        self.if_expr()
    }
    else if self.match_token(TokenType::KEYWHILE){
        self.while_expr()
    }
    else if self.match_token(TokenType::KEYCASE){
        self.case()
    }
    else if self.match_token(TokenType::KEYLET){
        self.let_expr()
    }
    else{
        self.error("Expected expression",self.peek().line);
        Expr::Error
    }
}

}