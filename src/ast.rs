use crate::token::Token;
use crate::token::TokenType::*;

/*
operator precedence from highest to lowest
.
@
~ bitwie not
isvoid 
* /
+ -
<= < = >=   // = is equality operator . => is more than operator.
not
<- =>
*/

/* 
cool syntax in bnf-like form as specified in  https://theory.stanford.edu/~aiken/software/cool/cool-manual.pdf
[] are optional
[[]] is used to show association of grammar symbols 

i have simplified the grammer a bit

program ::= [[class; ]]+
class ::= "class" TYPE ["inherits" TYPE] "{" [[feature";" ]]∗ "}"
feature ::= method|attribute
Method ::= ID( [ formal [[, formal]]∗ ] ) : TYPE { expr }
Attribute ::=  ID : TYPE [ <- expr ]
formal ::= ID : TYPE
expr ::= ID <- expr
| expr[@TYPE].ID( [ expr [[, expr]]∗ ] )
| ID( [ expr [[, expr]]∗ ] )
| if expr then expr else expr fi
| while expr loop expr pool
| { [[expr; ]]+}
| let ID : TYPE [ <- expr ] [[, ID : TYPE [ <- expr ]]]∗ in expr
| case expr of [[ID : TYPE => expr; ]]+esac
| new TYPE
| isvoid expr
| expr + expr
| expr − expr
| expr ∗ expr
| expr / expr
| ˜expr
| expr < expr
| expr <= expr
| expr = expr
| not expr
| (expr)
| ID
| integer
| string
| true
| false

*/

pub struct  Program{
    classes : Vec<Class>
}

pub struct Class{
    type_ : Token,
    inherits : Option<Token>,
    features : Vec<Feature>
}

type Type = Token;
type identifier = Token;

pub enum Feature {
    Method{
        id : identifier,
        parameters : Vec<Formal>,
    },
    Attribute{
        id : Token,
        type_ : Type,
        expr : Option<Expr>
    }
} 

pub struct Formal{
    id : identifier,
    type_ : Type
}

pub enum Expr{
    Assign{
        left : Box<Expr>,
        right : Box<Expr>
    },
    Not{
        expr : Box<Expr>
    },
    Comparison{
        left : Box<Expr>,
        operator : Token,
        right : Box<Expr>
    },
    Arithmetic{
        left : Box<Expr>,
        operator : Token,
        right : Box<Expr>
    },
    Factor{
        left : Box<Expr>,
        operator : Token,
        right : Box<Expr>
    },
    New{
        type_ : Token
    },
    IsVoid{
        expr : Box<Expr>
    },
    BitWiseNot{ //~ operator
        expr : Box<Expr>
    },
    DispatchSelection{ //@ operator
        expr : Box<Expr>,
        type_ : Type,
        id : Token,
        arguments : Vec<Expr>
    },
    Call{
        id : identifier,
        arguments : Vec<Expr>
    },
    StringLiteral{
        value : Token
    },
    IntgerLiteral{
        value : Token
    },
    BoolLiteral{
        value : Token
    },
    ID{
        id : identifier
    },
    Case{

    },
    If,
    While,
    Let,
    Block
}
