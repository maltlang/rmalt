use ast::Ast;

pub mod token;
pub mod lexer;

///## BNF Expression
/// '''bnf
/// expr ::= atom | String | Symbol | Tuple |
///
/// tuple ::= '[' [expr]* ']' ;
/// atom ::= UInt | Int | Float
/// default = Symbol | String | UInt | Int | Float
/// '''

pub fn parser(tf: &Vec<token::Token>) -> Result<Vec<Ast>, token::TokenPos>  {

}