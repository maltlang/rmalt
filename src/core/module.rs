use value::ast::Ast;

pub struct Module {
    path: String,

    asts: Vec<Ast>,
}