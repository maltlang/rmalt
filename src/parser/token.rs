use std::sync::Arc;

pub enum TokenValue {
    LP,
    RP,
    LMP,
    RMP,
    QUO,

    INT(i64),
    UINT(u64),
    FLOAT(f64),

    STRING(Arc<String>),
    SYMBOL(Arc<String>),
}

pub struct TokenPos {
    pub line: usize,
    pub col: usize,
}

pub struct Token {
    pub val: TokenValue,
    pub pos: TokenPos,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push('<');
        match self.val {
            TokenValue::LP => s.push('('),
            TokenValue::RP => s.push(')'),
            TokenValue::LMP => s.push('['),
            TokenValue::RMP => s.push(']'),
            TokenValue::QUO => s.push('\''),
            TokenValue::INT(ref x) => {
                s.push_str("int ");
                s.push_str(&x.to_string());
            }
            TokenValue::UINT(ref x) => {
                s.push_str("uint ");
                s.push_str(&x.to_string());
            }
            TokenValue::FLOAT(ref x) => {
                s.push_str("float ");
                s.push_str(&x.to_string());
            }
            TokenValue::STRING(ref x) => {
                s.push_str("string ");
                s.push_str(&x.to_string());
            }
            TokenValue::SYMBOL(ref x) => {
                s.push_str("symbol ");
                s.push_str(&x.to_string());
            }
        }
        s.push('>');
        s.push_str(&self.pos.line.to_string());
        s.push(':');
        s.push_str(&self.pos.col.to_string());

        s
    }
}