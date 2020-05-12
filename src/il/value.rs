use std::fmt::Display;
use super::Block;

#[derive(Debug)]
pub enum Value {
    // Basic types
    Int(isize),
    Uint(usize),
    Byte(u8),
    Char(char),
    Bool(bool),
    List(Vec<Value>),
    Function(usize, String, Block, Vec<(String, Value)>),
    // TODO: add object type

    Name(usize, String),

    // operations
    Greater(Box<Value>, Box<Value>),
    GreaterEqual(Box<Value>, Box<Value>),
    Less(Box<Value>, Box<Value>),
    LessEqual(Box<Value>, Box<Value>),
    Equal(Box<Value>, Box<Value>),

    And(Box<Value>, Box<Value>),
    Or(Box<Value>, Box<Value>),
    Not(Box<Value>),

    Add(Box<Value>, Box<Value>),
    Sub(Box<Value>, Box<Value>),
    Mul(Box<Value>, Box<Value>),
    Div(Box<Value>, Box<Value>),
    Modulo(Box<Value>, Box<Value>),

    BitwiseAnd(Box<Value>, Box<Value>),
    BitwiseOr(Box<Value>, Box<Value>),
    BitwiseXor(Box<Value>, Box<Value>),
    BitwiseNot(Box<Value>),
    BitwiseShift(Box<Value>, Box<Value>),
    BitwiseUnshift(Box<Value>, Box<Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Uint(i) => write!(f, "{}", i),
            Value::Byte(b) => write!(f, "{:02X}", b),
            Value::Char(c) => write!(f, "{}", c),
            Value::Bool(b) => write!(f, "{}", if *b {"true"} else {"false"}),
            Value::List(l) => write!(f, "[{}]", {
                let mut res = format!("{}", l[0]);
                for i in 1..l.len() {
                    res += &format!(", {}", l[i]);
                }
                res
            }),

            Value::Name(id, s) => write!(f, "({}#{})", s, id),

            Value::Function(id, name, _code, args) => write!(f, "({}#{})({})", name, id, args.iter().map(|x| {format!("{}: {}", x.0, x.1)}).collect::<Vec<String>>().join(", ")),

            // operations
            Value::Greater(lhs, rhs) => write!(f, "{} > {}", lhs, rhs),
            Value::GreaterEqual(lhs, rhs) => write!(f, "{} >= {}", lhs, rhs),
            Value::Less(lhs, rhs) => write!(f, "{} < {}", lhs, rhs),
            Value::LessEqual(lhs, rhs) => write!(f, "{} <= {}", lhs, rhs),
            Value::Equal(lhs, rhs) => write!(f, "{} == {}", lhs, rhs),

            Value::And(lhs, rhs) => write!(f, "{} && {}", lhs, rhs),
            Value::Or(lhs, rhs) => write!(f, "{} || {}", lhs, rhs),
            Value::Not(v) => write!(f, "!{}", v),

            Value::Add(lhs, rhs) => write!(f, "{} + {}", lhs, rhs),
            Value::Sub(lhs, rhs) => write!(f, "{} - {}", lhs, rhs),
            Value::Mul(lhs, rhs) => write!(f, "{} * {}", lhs, rhs),
            Value::Div(lhs, rhs) => write!(f, "{} / {}", lhs, rhs),
            Value::Modulo(lhs, rhs) => write!(f, "{} % {}", lhs, rhs),

            Value::BitwiseAnd(lhs, rhs) => write!(f, "{} & {}", lhs, rhs),
            Value::BitwiseOr(lhs, rhs) => write!(f, "{} | {}", lhs, rhs),
            Value::BitwiseXor(lhs, rhs) => write!(f, "{} ^ {}", lhs, rhs),
            Value::BitwiseNot(v) => write!(f, "~{}",v),
            Value::BitwiseShift(lhs, rhs) => write!(f, "{} << {}", lhs, rhs),
            Value::BitwiseUnshift(lhs, rhs) => write!(f, "{} >> {}", lhs, rhs),
        }?;
        Ok(())
    }
}