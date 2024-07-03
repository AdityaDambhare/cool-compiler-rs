#[derive(Debug,Copy,Clone,PartialEq)]
pub enum TokenType{
    //keywords
    KEYCLASS,KEYIF,KEYELSE,KEYTHEN,KEYFI,KEYIN,KEYINHERITS,KEYLET,KEYLOOP,KEYPOOL,KEYWHILE,
    KEYCASE,KEYESAC,KEYNEW,KEYOF,KEYTRUE,KEYFALSE,KEYDELETE,
    //All keywords are case-insensitive except true and false

    INTEGER,IDENTIFIER,STRING,

    LEFTPAREN,RIGHTPAREN,LEFTBRACE,RIGHTBRACE,SEMICOLON,COLON,COMMA,

    //operators
    DOT,AT,TILDA,ISVOID,STAR,SLASH,PLUS,MINUS,NOT,LESSTHAN,MORETHAN,ASSIGN,EQUALITY,LESSEQUAL,MOREEQUAL,RARROW,

    EOF,
    ERROR
}
#[derive(Debug,Clone)]
pub struct Token{
   pub line : usize,
   pub lexeme : String,
  pub  tokentype : TokenType,
   pub  literal : Option<String>
}
impl Token{
    pub fn new(line:usize,lexeme:String,tokentype:TokenType,literal:Option<String>)->Token{
        Token { line, lexeme, tokentype ,literal  }
    }
    
    pub fn keyword(lexeme:&str)->Option<TokenType>{
        match lexeme{
        "delete" => Some(TokenType::KEYDELETE),
        "isvoid" => Some(TokenType::ISVOID),
        "class" => Some(TokenType::KEYCLASS),
        "if" => Some(TokenType::KEYIF),
        "else" => Some(TokenType::KEYELSE),
        "then" => Some(TokenType::KEYTHEN),
        "fi" => Some(TokenType::KEYFI),
        "in" => Some(TokenType::KEYIN),
        "let" => Some(TokenType::KEYLET),
        "loop" => Some(TokenType::KEYLOOP),
        "pool" => Some(TokenType::KEYPOOL),
        "while" => Some(TokenType::KEYWHILE),
        "case" => Some(TokenType::KEYCASE),
        "esac" => Some(TokenType::KEYESAC),
        "new" => Some(TokenType::KEYNEW),
        "of" => Some(TokenType::KEYOF),
        "not" => Some(TokenType::NOT),
        "true" => Some(TokenType::KEYTRUE),
        "false" => Some(TokenType::KEYFALSE),
        "inherits" => Some(TokenType::KEYINHERITS),
            _=> None
        }
    }
}

