#[derive(Debug)]
pub enum TokenType{
    //keywords
    KEYCLASS,KEYIF,KEYELSE,KEYTHEN,KEYFI,KEYIN,KEYINHERITS,KEYLET,KEYLOOP,KEYPOOL,KEYWHILE,
    KEYCASE,KEYESAC,KEYNEW,KEYOF,KEYTRUE,KEYFALSE,
    //All keywords are case-insensitive except true and false

    INTEGER,IDENTIFIER,STRING,

    LEFTPAREN,RIGHTPAREN,LEFTBRACE,RIGHTBRACE,SEMICOLON,COLON,

    //operators
    DOT,AT,TILDA,ISVOID,STAR,SLASH,PLUS,MINUS,RARROW,LARROW,NOT,EQUAL,LESSTHAN,MORETHAN,ASSIGN,

    EOF,
    ERROR
}
#[derive(Debug)]
pub struct Token{
    line : usize,
    lexeme : String,
    tokentype : TokenType,
    literal : Option<String>
}
impl Token{
    pub fn new(line:usize,lexeme:String,tokentype:TokenType,literal:Option<String>)->Token{
        Token { line, lexeme, tokentype ,literal  }
    }
    pub fn keyword(lexeme:&str)->Option<TokenType>{
        match lexeme{
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

impl TokenType{
    pub fn to_string(self)->String{
        match self{
            TokenType::KEYCLASS=>"KEYCLASSS",
            TokenType::KEYIF=>"KEYIF",
            TokenType::KEYELSE=>"KEYELSE",
            TokenType::KEYTHEN=>"KEYTHEN",
            TokenType:: KEYFI=>"KEYFI",
            TokenType:: KEYIN=>"KEYIN",
            TokenType:: KEYINHERITS=>"KEYINHERITS",
            TokenType:: STRING =>"STRING",
            TokenType:: KEYLET=>"KEYLET",
            TokenType::  KEYLOOP=>"KEYLOOP",
            TokenType::  KEYPOOL=>"KEYPOOL",
            TokenType::  KEYWHILE=>"KEYWHILE",
            TokenType::  KEYCASE=>"KEYCASE",
            TokenType::  KEYESAC=>"KEYESAC",
            TokenType::  KEYNEW=>"KEYNEW",
            TokenType::  KEYOF=>"KEYOF",
            
            TokenType::  KEYTRUE=>"KEYTRUE",
            TokenType::  KEYFALSE=>"KEYFALSE",
            TokenType::  INTEGER=>"INTEGER",
            TokenType::  IDENTIFIER=>"IDENTIFIER",
            
            TokenType::  LEFTPAREN=>"LEFTPAREN",
            TokenType::  RIGHTPAREN=>"RIGHTPAREN",
            TokenType::  LEFTBRACE=>"LEFTBRACE",
            TokenType::  RIGHTBRACE=>"RIGHTBRACE",
            TokenType::  SEMICOLON=>"SEMICOLON",
            TokenType::  COLON=>"COLON",
            TokenType::  DOT=>"DOT",
            TokenType::  AT=>"AT",
            TokenType::  TILDA=>"TILDA",
            TokenType::  ISVOID=>"ISVOID",
            TokenType::  STAR=>"STAR",
            TokenType::  SLASH=>"SLASH",
            TokenType:: PLUS=>"PLUS",
            TokenType:: MINUS=>"MINUS",
            TokenType::  LARROW=>"LARROW",
            TokenType::  RARROW=>"RARROW",
            TokenType::  MORETHAN=>"MORETHAN",
            TokenType::  NOT=>"NOT",
            TokenType::  EQUAL=>"EQUAL",
            TokenType::  LESSTHAN=>"LESSTHAN",
            TokenType::  ASSIGN=>"ASSIGN",
            TokenType::  EOF=>"EOF",
            TokenType::  ERROR=>"ERROR"
        }.to_string()
    }
  
}