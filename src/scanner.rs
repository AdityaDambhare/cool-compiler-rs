use crate::token::*;
use crate::token::TokenType::*;

pub struct Scanner{
    had_error: bool,
    line:  usize,
    start: usize,
    current: usize,
    source : String
}

impl Scanner{

    pub fn new(source:String)->Scanner{
        Scanner{
            had_error:false,
            line:1usize,
            source:source.to_string(),
            current:0usize,
            start:0usize
        }
    }

    fn error(&mut self,line:usize,message:&str){
        eprintln!("[line {}] Error: {}",line,message);
        self.had_error = true;
    }

    pub fn scan_tokens(&mut self)->Result<Vec<Token>,&str>{
        self.line  = 1;
        let mut tokens : Vec<Token> = Vec::new();
        

        while !self.eof() {
            self.start = self.current;
            let scanned_token = self.scan_token();
            match scanned_token{
                Some(token) => tokens.push(token),
                None => continue
            }
        }

        tokens.push(
            Token::new(
                self.line,
                "\0".to_string(),
                TokenType::EOF,
                None
            )
        );
        if self.had_error{
            Err("Error in scanning")
        }
        else{
            Ok(tokens)
        }
    }

   fn add_token(&mut self,tokentype:TokenType,literal:Option<String>)->Option<Token>{
        Some(Token::new(self.line,self.source[self.start..self.current].trim().to_string(),tokentype,literal))
   }

    fn scan_token(&mut self)->Option<Token>{
        let c = self.advance();
        match  c {
            '\n' => {
                self.line = self.line + 1;
                None
            },
            '(' => {
                if self.check_next('*'){
                    self.block_comment();
                    None
                }
                else{
                    self.add_token(LEFTPAREN, None)
                }
            
            },
            ',' => self.add_token(COMMA, None),
            ')' => self.add_token(RIGHTPAREN, None),
            '{' => self.add_token(LEFTBRACE, None),
            '}' => self.add_token(RIGHTBRACE, None),
            ';' => self.add_token(SEMICOLON, None),
            ':' => self.add_token(COLON, None),
            '.' => self.add_token(DOT, None),
            '@' => self.add_token(AT, None),
            '~' => self.add_token(TILDA, None),
            '+' =>self.add_token(PLUS, None),
            '-' =>{
                if self.check_next('-'){
                    self.comment();
                    None
                }
                else{
                    self.add_token(MINUS, None)
                }
            },
            '*' =>self.add_token(STAR, None),
            '/' =>self.add_token(SLASH, None) ,
            '=' =>{
                if self.check_next('>'){
                    self.advance();
                    self.add_token(RARROW, None)
                }
                else{
                    self.add_token(EQUALITY, None)
                }
            },
            '>' =>
            {
                if self.check_next('='){
                    self.advance();
                    self.add_token(MOREEQUAL, None)
                }
                else{
                    self.add_token(MORETHAN, None)
                }
            },
            '<' =>{
                if self.check_next('='){
                    self.advance();
                    self.add_token(LESSEQUAL, None)
                }
                else if self.check_next('-'){
                    self.advance();
                    self.add_token(ASSIGN, None)
                }
                else{
                    self.add_token(LESSTHAN, None)
                }
            },
            '\"'=>{self.advance();self.string_token()},
            ' ' | '\r' | '\t' => None,
            _ =>{
                if Self::is_alpha(c) {
                    self.identifier()
                }
                else if Self::is_digit(c) {
                    self.number()
                }
               
                else{
                    self.error(self.line,&format!("Unexpected character '{}' ",c));
                   None
                }
            }
         }

    }

    fn peek(&mut self)->char{
        match self.source.chars().nth(self.current-1){
            None =>'`',
            Some(value) => value
        }
    }
    fn peek_next(&mut self)->char{
        match self.source.chars().nth(self.current){
            None =>'`',
            Some(value) => value
        }
    }

    fn check(&mut self,c:char)->bool{
      !self.eof() && c == self.peek() 
    }

    fn check_next(&mut self,c:char)->bool{
       !self.eof()&& c==self.peek_next()
    }

    fn advance(&mut self)->char{
        self.current = self.current + 1;
        match self.source.chars().nth(self.current-1){
            None =>'`',
            Some(value) => value
        }
    }

    fn identifier(&mut self)->Option<Token>{
        while !self.eof() && Self::is_alphanumeric(self.peek()){
            self.advance();
        }
        self.current = self.current-1;
        let lexeme :&str = self.source[self.start..self.current].trim();
        let tokentype :TokenType = match Token::keyword(lexeme.to_lowercase().as_str()) {
          Some(value) => value,
          None => IDENTIFIER  
        };
        self.add_token(tokentype, Some(lexeme.to_string()))
    }

    fn number(&mut self)->Option<Token>{
        while !self.eof() && Self::is_digit(self.peek()){
            self.advance();
        }
        self.current = self.current - 1;
        self.add_token(INTEGER, Some(self.source[self.start..self.current].to_string()))
    }

    fn string_token(&mut self)->Option<Token>{
        while !self.eof() && !self.check('\"') {
            if self.check('\\'){
                self.advance();
            }
            if self.check('\0') {self.error(self.line,"null character in string");}
            if self.check('\n'){
                self.line = self.line + 1;
            }
            self.advance();
        }
        if self.eof() {
            self.error(self.line,"Unterminated string");
            return  self.add_token(ERROR, None);
        }
       self.start = self.start + 1;
       self.current = self.current - 1;
       let tok = self.add_token(STRING, Some(self.source[self.start..self.current].to_string()));
       self.advance();
       tok
    }

    fn is_digit(c : char)->bool{
        c>='0' && c<='9'    
    }
    fn is_alpha(c:char)->bool{
        (c>='a' && c<='z')||(c=='_')||(c>='A' && c<='Z')
    }
    fn is_alphanumeric(c:char)->bool{
        Self::is_alpha(c)||Self::is_digit(c)
    }
    fn comment(&mut self){
        while !self.eof() && !self.check('\n') {
            self.advance();
        }   
        if self.check('\n'){self.line = self.line + 1;}
        
    }
    fn block_comment(&mut self){
        let line = self.line;
        self.advance();
        while !self.eof(){
            if self.check('\n'){
                self.line = self.line + 1;
            }
            if self.check('(')&&self.check_next('*'){
                self.block_comment();
            }
            if self.check('*') && self.check_next(')'){
                self.advance();
                return;
            }
            self.advance();
            
        }
        self.error(line,"Unterminated block comment");
        
    }
    
    fn eof(&mut self)->bool{
        self.current >= self.source.len()  
    }

}