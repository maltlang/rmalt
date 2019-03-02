use runtime::*;
use parser::parserc::*;

pub fn parse_atom(ss: &StrStream) -> ParserOut {
    ((ParserC::new_f(parse_none)
        | ParserC::new_f(parse_bool)
        | ParserC::new_f(parse_char_text)
        | ParserC::new_f(parse_string_text)
        | ParserC::new_f(parse_rational)
        | ParserC::new_f(parse_int)
        | ParserC::new_f(parse_uint)
        | ParserC::new_f(parse_symbol)).f)(ss)
}
/*
    if let Some(x) = parse_none(ss) {
        Some(x)
    } else if let Some(x) = parse_bool(ss) {
        Some(x)
    } else if let Some(x) = parse_char_text(ss) {
        Some(x)
    } else if let Some(x) = parse_string_text(ss) {
        Some(x)
    } else if let Some(x) = parse_rational(ss) {
        Some(x)
    } else if let Some(x) = parse_int(ss) {
        Some(x)
    } else if let Some(x) = parse_uint(ss) {
        Some(x)
    } else if let Some(x) = parse_symbol(ss) {
        Some(x)
    } else {
        None
    }
*/

pub fn parse_none(ss: &StrStream) -> ParserOut {
    if let Some((_, tail)) = parse_string(":none")(ss) {
        Some((Some(Ast {
            val: Tree::Value(Value::None),
            col: ss.col,
            lin: ss.lin
        }), parse_empty(&tail)?.1))
    } else {
        None
    }
}

pub fn parse_bool(ss: &StrStream) -> ParserOut {
    if let Some((_, tail)) = parse_string(":true")(ss) {
        Some((Some(Ast {
            val: Tree::Value(Value::Bool(true)),
            col: ss.col,
            lin: ss.lin
        }), parse_empty(&tail)?.1))
    } else if let Some((_, tail)) = parse_string(":false")(ss) {
        Some((Some(Ast {
            val: Tree::Value(Value::Bool(false)),
            col: ss.col,
            lin: ss.lin
        }), parse_empty(&tail)?.1))
    } else {
        None
    }
}