use crate::token::Token;


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
| case expr of [ID : TYPE => expr]]+esac
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
        type_ : Type,
        parameters : Vec<Formal>,
        body : Expr
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
#[derive(Debug)]
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
    },
    
    Dot{
        expr : Box<Expr>,
        id : identifier
    },
    Call{
        id : identifier,
        arguments : Vec<Expr>
    },
    StringLiteral{
        value : Token
    },
    IntegerLiteral{
        value : Token
    },
    BoolLiteral{
        value : Token
    },
    ID{
        id : identifier
    },
    Case{
        expr : Box<Expr>,
        branches : Vec<Expr>
    },
    Branch{
        id : identifier,
        type_ : Type,
        expr : Box<Expr>
    }
    ,
    If{
        Condition : Box<Expr>,
        Then : Box<Expr>,
        Else : Box<Expr> 
    },
    While{
        Condition : Box<Expr>,
        Loop : Box<Expr>
    },
    Let{

        declarations : Vec<Expr>,//Declaration
        body : Box<Expr>
    },
    Declaration{
        id : identifier,
        type_ : Type,
        expr : Box<Option<Expr>>
    },
    Block{
        exprs : Vec<Expr>
    },
    Grouping{
        expr : Box<Expr>
    },
    Error{
        message : String
    
    }
}

impl Program{
    pub fn new(classes:Vec<Class>)->Program{
        Program{classes}
    }
}

impl Class{
    pub fn new(type_:Token, inherits:Option<Token>, features:Vec<Feature>)->Class{
        Class{type_,inherits,features}
    }
}

impl Feature{
    pub fn new_method(id:identifier,type_:Type,parameters:Vec<Formal>,body:Expr)->Feature{
        Feature::Method{id,type_,parameters,body}
    }
    pub fn new_attribute(id:Token,type_:Type,expr:Option<Expr>)->Feature{
        Feature::Attribute{id,type_,expr}
    }
}

impl Formal{
    pub fn new(id:identifier,type_:Type)->Formal{
        Formal{id,type_}
    }
}

impl Expr{
    pub fn Not(expr:Expr)->Expr{
        Expr::Not{expr:Box::new(expr)}
    }
    pub fn Assign(left:Expr,right:Expr)->Expr{
        Expr::Assign{left:Box::new(left),right:Box::new(right)}
    }
    pub fn Comparison(left:Expr,operator:Token,right:Expr)->Expr{
        Expr::Comparison{left:Box::new(left),operator,right:Box::new(right)}
    }
    pub fn Arithmetic(left:Expr,operator:Token,right:Expr)->Expr{
        Expr::Arithmetic{left:Box::new(left),operator,right:Box::new(right)}
    }
    pub fn Factor(left:Expr,operator:Token,right:Expr)->Expr{
        Expr::Factor{left:Box::new(left),operator,right:Box::new(right)}
    }
    pub fn New(type_:Token)->Expr{
        Expr::New{type_}
    }
    pub fn IsVoid(expr:Expr)->Expr{
        Expr::IsVoid{expr:Box::new(expr)}
    }
    pub fn BitWiseNot(expr:Expr)->Expr{
        Expr::BitWiseNot{expr:Box::new(expr)}
    }
    pub fn Grouping(expr:Expr)->Expr{
        Expr::Grouping{expr:Box::new(expr)}
    }
    pub fn DispatchSelection(expr:Expr,type_:Type)->Expr{
        Expr::DispatchSelection{expr:Box::new(expr),type_}
    }
    pub fn Dot(expr:Expr,id:identifier)->Expr{
        Expr::Dot{expr:Box::new(expr),id}
    }
    pub fn Call(id:identifier,arguments:Vec<Expr>)->Expr{
        Expr::Call{id,arguments}
    }
    pub fn StringLiteral(value:Token)->Expr{
        Expr::StringLiteral{value}
    }
    pub fn IntegerLiteral(value:Token)->Expr{
        Expr::IntegerLiteral{value}
    }
    pub fn BoolLiteral(value:Token)->Expr{
        Expr::BoolLiteral{value}
    }
    pub fn Error(message:String)->Expr{
        Expr::Error{message}
    }
}
