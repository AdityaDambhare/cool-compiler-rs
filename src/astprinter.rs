

use crate::ast::*;
use crate::token::*;
pub struct AstPrinter ;
impl AstPrinter
{
    pub fn print_program(program:&Program)->String{
        let mut printer = AstPrinter{};
        program.accept(&mut printer)
    }
}
impl Visitor<String> for AstPrinter{
    
    fn visit_program(&mut self,classes:&Vec<Class>)->String{
        let mut result = String::new();
        for class in classes{
            result.push_str(&class.accept(self));
            result.push_str("\n");
        }
        result
    }

    fn visit_class(&mut self,type_:&Token,inherits:&Option<Token>,features:&Vec<Feature>)->String{
        let mut result = String::new();
        result.push_str(&format!("class {} ",type_.lexeme));
        if let Some(inherits) = inherits{
            result.push_str(&format!("inherits {} ",inherits.lexeme));
        }
        result.push_str("{\n");
        for feature in features{
            result.push_str(&feature.accept(self));
            result.push_str(";\n");
        }
        result.push_str("};");
        result
    }

    fn visit_attribute(&mut self,id:&Token,type_:&Token,expr:&Option<Expr>)->String {
        let mut result = String::new();
        result.push_str(&format!("{} : {}",id.lexeme,type_.lexeme));
        if let Some(expr) = expr{
            result.push_str(" <- (");
            result.push_str(&expr.accept(self));
            result.push_str(")");
        }

        result
    }

    fn visit_method(&mut self,id:&Token,type_:&Token,parameters:&Vec<Formal>,body:&Expr)->String {
        let mut result = String::new();
        result.push_str(&format!("{} (",id.lexeme));
        for parameter in parameters{
            result.push_str(&parameter.accept(self));
            result.push_str(",");
        }
        if result.ends_with(","){
            result.pop();
        }
        result.push_str(") : ");
        result.push_str(&type_.lexeme);
        result.push_str(" {\n");
        result.push_str(&body.accept(self));
        result.push_str("\n }");
        result
    }

    fn visit_formal(&mut self,id:&Token,type_:&Token)->String{
        format!("{} : {}",id.lexeme,type_.lexeme)
    }


    fn visit_expr(&mut self,expr:&Expr)->String{
        expr.accept(self)
    }

    fn visit_assign(&mut self,left:&Expr,right:&Expr)->String {
        format!("{} <- {}",left.accept(self),right.accept(self))
    }

    fn visit_arithmetic(&mut self,left:&Expr,operator:&Token,right:&Expr)->String {
        format!("({} {} {})",left.accept(self),operator.lexeme,right.accept(self))
    }

    fn visit_factor(&mut self,left:&Expr,operator:&Token,right:&Expr)->String {
        format!("({} {} {})",left.accept(self),operator.lexeme,right.accept(self))
    }

    fn visit_comparison(&mut self,left:&Expr,operator:&Token,right:&Expr)->String {
        format!("({} {} {})",left.accept(self),operator.lexeme,right.accept(self))
    }

    fn visit_bitwise_not(&mut self,bitwise_not_expr:&Expr)->String {
        format!("(~{})",bitwise_not_expr.accept(self))
    }

    fn visit_new(&mut self,new_expr:&Token)->String {
        format!("new {}",new_expr.lexeme)
    }

    fn visit_delete(&mut self,delete_expr:&Expr)->String {
        format!("delete {}",delete_expr.accept(self))
    }

    fn visit_isvoid(&mut self,isvoid_expr:&Expr)->String {
        format!("isvoid {}",isvoid_expr.accept(self))
    }

    fn visit_grouping(&mut self,grouping_expr:&Expr)->String {
        format!("({})",grouping_expr.accept(self))
    }

    fn visit_block(&mut self,block_expr:&Vec<Expr>)->String {
        let mut result = String::new();
        result.push_str("  {\n");
        for expr in block_expr{
            result.push_str(&expr.accept(self));
            result.push_str(";\n");
        }
        result.push_str("  }");
        result
    }

    fn visit_case(&mut self,condition:&Expr,branches:&Vec<Expr>)->String {
        let mut result = String::new();
        result.push_str("case ");
        result.push_str(&condition.accept(self));
        result.push_str(" of\n");
        for branch in branches{
            result.push_str(&branch.accept(self));
            result.push_str("\n");
        }
        result.push_str("esac\n");
        result
    }

    fn visit_branch(&mut self,id:&Token,type_:&Token,expr:&Expr)->String {
        format!("{} : {} => {}",id.lexeme,type_.lexeme,expr.accept(self))
    }

    fn visit_while(&mut self,condition:&Expr,body:&Expr)->String {
        format!("while {} loop\n{}\npool\n",condition.accept(self),body.accept(self))
    }

    fn visit_not(&mut self,not_expr:&Expr)->String {
        format!("(not {})",not_expr.accept(self))
    }

    fn visit_if(&mut self,condition:&Expr,body:&Expr,else_expr:&Expr)->String {
        format!("if {} then {}\nelse\n{}\nfi",condition.accept(self),body.accept(self),else_expr.accept(self))
    }

    fn visit_let(&mut self,declarations:&Vec<Expr>,body:&Expr)->String {
        let mut result = String::new();
        result.push_str("let\n");
        for declaration in declarations{
            result.push_str(&declaration.accept(self));
            result.push_str("\n");
        }
        result.push_str("in\n");
        result.push_str(&body.accept(self));
        result.push_str("\n");
        result
    }

    fn visit_declaration(&mut self,id:&Token,type_:&Token,expr:&Option<Expr>)->String {
        let mut result = String::new();
        result.push_str(&format!("{} : {}",id.lexeme,type_.lexeme));
        if let Some(expr) = expr{
            result.push_str(" <- ");
            result.push_str(&expr.accept(self));
        }
        result
    }

    fn visit_boolliteral(&mut self,boolliteral:&Token)->String {
        boolliteral.lexeme.clone()
    }

    fn visit_id(&mut self,id:&Token)->String {
        id.lexeme.clone()
    }

    fn visit_integerliteral(&mut self,integerliteral:&Token)->String {
        integerliteral.lexeme.clone()
    }

    fn visit_stringliteral(&mut self,stringliteral:&Token)->String {
        format!("\"{}\"",stringliteral.lexeme.clone())
    }
    fn visit_dispatch(&mut self,target:&Option<Token>,expr:&Expr,method_name:&Option<Token>,arguments:&Vec<Expr>)->String {
        let mut result = String::new();
        result.push_str(&expr.accept(self));
        match target {
            Some(tok) => {result.push_str("@");result.push_str(&tok.lexeme.clone())},
            None => ()
        };
        match method_name {
            Some(tok) => {result.push_str(".");result.push_str(&tok.lexeme.clone())},
            None => ()
        };
        result.push_str("(");
        for expr in arguments{
            result.push_str(&expr.accept(self));
            result.push_str(",");
        }
        if result.ends_with(","){
            result.pop();
        }
        result.push_str(")");
        result
    }
    fn not_implemented(&mut self)->String {
        String::from("Not implemented")
    }
    
}