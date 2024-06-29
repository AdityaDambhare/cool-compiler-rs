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

pub trait  Visitor<T>{
    fn visit_program(&mut self,classes:&Vec<Class>)->T;
    fn visit_class(&mut self,type_:&Token,inherits:&Option<Token>,feature:&Vec<Feature>)->T;
    fn visit_method(&mut self,id:&Token,type_:&Token,parameters:&Vec<Formal>,body:&Expr)->T;
    fn visit_attribute(&mut self,id:&Token,type_:&Token,expr:&Option<Expr>)->T;
    fn visit_formal(&mut self,id:&Token,type_:&Token)->T;
    fn visit_expr(&mut self,expr:&Expr)->T;
    fn visit_branch(&mut self,id:&Token,type_:&Token,expr:&Expr)->T;
    fn visit_declaration(&mut self,id:&Token,type_:&Token,expr:&Option<Expr>)->T;
    fn visit_block(&mut self,exprs:&Vec<Expr>)->T;
    fn visit_let(&mut self,declarations:&Vec<Expr>,body:&Expr)->T;
    fn visit_while(&mut self,condition:&Expr,body:&Expr)->T;
    fn visit_if(&mut self,condition:&Expr,body:&Expr,else_expr:&Expr)->T;
    fn visit_case(&mut self,condition:&Expr,branches:&Vec<Expr>)->T;
    fn visit_arithmetic(&mut self,left:&Expr,operator:&Token,right:&Expr)->T;
    fn visit_comparison(&mut self,left:&Expr,operator:&Token,right:&Expr)->T;
    fn visit_factor(&mut self,left:&Expr,operator:&Token,right:&Expr)->T;
    fn visit_assign(&mut self,left:&Expr,right:&Expr)->T;
    fn visit_not(&mut self,not_expr:&Expr)->T;
    fn visit_new(&mut self,new_expr:&Token)->T;
    fn visit_isvoid(&mut self,isvoid_expr:&Expr)->T;
    fn visit_bitwise_not(&mut self,bitwise_not_expr:&Expr)->T;
    fn visit_grouping(&mut self,grouping_expr:&Expr)->T;
    fn not_implemented(&mut self)->T;
    fn visit_stringliteral(&mut self,stringliteral:&Token)->T;
    fn visit_integerliteral(&mut self,integerliteral:&Token)->T;
    fn visit_boolliteral(&mut self,boolliteral:&Token)->T;
    fn visit_id(&mut self,id:&Token)->T;
}


#[derive(Debug)]
pub struct  Program{
    classes : Vec<Class>
}
#[derive(Debug)]
pub struct Class{
    type_ : Token,
    inherits : Option<Token>,
    features : Vec<Feature>
}

type Type = Token;
type identifier = Token;
#[derive(Debug)]
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
#[derive(Debug)]
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
    Error
}

impl Program{
    pub fn new(classes:Vec<Class>)->Program{
        Program{classes}
    }
    pub fn accept<T>(&self,visitor:&mut dyn Visitor<T>)->T{
        visitor.visit_program(&self.classes)
    }
}

impl Class{
    pub fn new(type_:Token, inherits:Option<Token>, features:Vec<Feature>)->Class{
        Class{type_,inherits,features}
    }
    pub fn accept<T>(&self,visitor:&mut dyn Visitor<T>)->T{
        visitor.visit_class( &self.type_, &self.inherits, &self.features)
    }
}

impl Feature{
    pub fn new_method(id:identifier,type_:Type,parameters:Vec<Formal>,body:Expr)->Feature{
        Feature::Method{id,type_,parameters,body}
    }
    pub fn new_attribute(id:Token,type_:Type,expr:Option<Expr>)->Feature{
        Feature::Attribute{id,type_,expr}
    }
    pub fn accept<T>(&self,visitor:&mut dyn Visitor<T>)->T{
        match self{
            Feature::Method{id,type_,parameters,body} => visitor.visit_method(id,type_,parameters,body),
            Feature::Attribute{id,type_,expr} => visitor.visit_attribute(id,type_,expr)
        }
    }
}

impl Formal{
    pub fn new(id:identifier,type_:Type)->Formal{
        Formal{id,type_}
    }
    pub fn accept<T>(&self,visitor:&mut dyn Visitor<T>)->T{
        visitor.visit_formal(&self.id,&self.type_)
    }
}

impl Expr{
    pub fn accept<T>(&self,visit:&mut dyn Visitor<T>)->T{
        match self{
            Expr::Assign{left,right} => visit.visit_assign(left,right),
            Expr::Not{expr} => visit.visit_not(expr),
            Expr::Comparison{left,operator,right} => visit.visit_comparison(left,operator,right),
            Expr::Arithmetic{left,operator,right} => visit.visit_arithmetic(left,operator,right),
            Expr::Factor{left,operator,right} => visit.visit_factor(left,operator,right),
            Expr::New{type_} => visit.visit_new(type_),
            Expr::IsVoid{expr} => visit.visit_isvoid(expr),
            Expr::BitWiseNot{expr} => visit.visit_bitwise_not(expr),
            Expr::StringLiteral{value} => visit.visit_stringliteral(value),
            Expr::IntegerLiteral{value} => visit.visit_integerliteral(value),
            Expr::BoolLiteral{value} => visit.visit_boolliteral(value),
            Expr::ID{id} => visit.visit_id(id),
            Expr::Case{expr,branches} => visit.visit_case(expr ,branches),
            Expr::Branch{id,type_,expr} => visit.visit_branch(id,type_,expr),
            Expr::If{Condition,Then,Else} => visit.visit_if(&Condition,Then,Else),
            Expr::While{Condition,Loop} => visit.visit_while(Condition,Loop),
            Expr::Let{declarations,body} => visit.visit_let(declarations,body),
            Expr::Declaration{id,type_,expr} => visit.visit_declaration(id,type_,expr),
            Expr::Block{exprs} => visit.visit_block(exprs),
            Expr::Grouping{expr} => visit.visit_grouping(expr),
            _ => visit.not_implemented()
        }
    }
}


impl Expr{
    pub fn IF_EXPR(Condition:Expr,Then:Expr,Else:Expr)->Expr{
        Expr::If{Condition:Box::new(Condition),Then:Box::new(Then),Else:Box::new(Else)}
    }
    pub fn BLOCK_EXPR(exprs:Vec<Expr>)->Expr{
        Expr::Block{exprs}
    }
    pub fn WHILE_EXPR(condition:Expr,body:Expr)->Expr{
        Expr::While{Condition:Box::new(condition),Loop:Box::new(body)}
    }
    pub fn LET_EXPR(declarations:Vec<Expr>,body:Expr)->Expr{
        Expr::Let{declarations,body:Box::new(body)}
    }
    pub fn CASE_EXPR(expr:Expr,branches:Vec<Expr>)->Expr{
        Expr::Case{expr:Box::new(expr),branches}
    }
    pub fn Declaration(id:identifier,type_:Type,expr:Option<Expr>)->Expr{
        Expr::Declaration{id,type_,expr:Box::new(expr)}
    }
    pub fn ID(id:identifier)->Expr{
        Expr::ID{id}
    }
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
    
}
