use std::fmt::Display;
use std::slice::Iter;

use super::{Type, Value};

#[derive(Debug, Clone)]
pub struct Block {
    v: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    If(Value, Block, Vec<(Value, Block)>, Option<Block>),
    FnDeclare(String, Block, Vec<(String, Type)>, Type),
    Val(Value),
    VarDeclare(String, Type, Value),
    VarAssign(String, Value),

}

impl Block {
    pub fn new() -> Block {
        Block { v: Vec::new() }
    }

    pub fn add(&mut self, s: Statement) {
        self.v.push(s)
    }

    pub fn iter(&self) -> Iter<'_, Statement> {
        self.v.iter()
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        writeln!(f, "{{")?;
        for s in &self.v {
            writeln!(f, "{}", s)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Statement::If(condition, code, elseif, r#else) => {
                write!(f, "if ( {} ) {}", condition, code)?;
                for (cond, c) in elseif {
                    write!(f, "else if ( {} ) {}", cond, c)?;
                }

                if let Some(e) = r#else {
                    write!(f, "else {}", e)?;
                }
            }
            Statement::FnDeclare(name, code, args, res) => write!(
                f,
                "{}({}): {} {}",
                name,
                args.iter()
                    .map(|x| { format!("{}: {}", x.0, x.1) })
                    .collect::<Vec<String>>()
                    .join(", "),
                res,
                code
            )?,
            Statement::Val(v) => write!(f, "{}", v)?,
            Statement::VarDeclare(name, t, v) => write!(f, "let {}: {} = {}", name, t, v)?,
            Statement::VarAssign(name, v) => write!(f, "{} = {}", name, v)?,
        };
        Ok(())
    }
}
