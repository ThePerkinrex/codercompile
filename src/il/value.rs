use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Value {
    // Basic values
    Int(isize),
    Uint(usize),
    Byte(u8),
    Char(char),
    Bool(bool),
    List(Vec<Value>),

    Custom(Type, Vec<(String, Value)>),

    Type(Type),

    Name(String),

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

    FnCall(String, Vec<Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Uint(i) => write!(f, "{}", i),
            Value::Byte(b) => write!(f, "{:02X}", b),
            Value::Char(c) => write!(f, "{}", c),
            Value::Bool(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            Value::List(l) => write!(f, "[{}]", {
                let mut res = format!("{}", l[0]);
                for i in 1..l.len() {
                    res += &format!(", {}", l[i]);
                }
                res
            }),

            Value::Custom(t, v) => write!(
                f,
                "{} {{\n{}\n}}",
                t,
                v.iter()
                    .map(|(name, value)| { format!("{} = {}", name, value) })
                    .collect::<Vec<String>>()
                    .join("\n")
            ),

            Value::Type(t) => write!(f, "{}", t),

            Value::Name(s) => write!(f, "{}", s),

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
            Value::BitwiseNot(v) => write!(f, "~{}", v),
            Value::BitwiseShift(lhs, rhs) => write!(f, "{} << {}", lhs, rhs),
            Value::BitwiseUnshift(lhs, rhs) => write!(f, "{} >> {}", lhs, rhs),

            Value::FnCall(name, args) => write!(f, "{}({})", name, args.iter().map(|x| {format!("{}", x)}).collect::<Vec<String>>().join(", "))
        }?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    // Basic types
    Null,
    Int,
    Uint,
    Byte,
    Char,
    Bool,
    List,
    Array(Box<Type>),

    Custom(String),
    Function(Vec<Type>, Box<Type>),
    Union(Vec<Type>)
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Type::Array(t) => write!(f, "[{}]", t)?,
            Type::Custom(c) => write!(f, "{}", c)?,
            Type::Function(args, res) => write!(
                f,
                "({}): {}",
                args.iter()
                    .map(|x| { format!("{}", x) })
                    .collect::<Vec<String>>()
                    .join(", "),
                res
            )?,
            _ => write!(f, "{:?}", self)?,
        };
        Ok(())
    }
}
