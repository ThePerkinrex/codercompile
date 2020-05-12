use std::fmt::Display;

use super::Value;

#[derive(Debug)]
pub struct Block {
    v: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    FnCall(usize, String, Vec<Value>),
    If(Value, Block, Vec<(Value, Block)>, Option<Block>),
}

impl Block {
    pub fn new() -> Block {
        Block { v: Vec::new() }
    }

    pub fn add(&mut self, s: Statement) {
        self.v.push(s)
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
            Statement::FnCall(id, name, args) => write!(
                f,
                "({}#{})({})",
                name,
                id,
                args.iter()
                    .map(|x| { format!("{}", x) })
                    .collect::<Vec<String>>()
                    .join(", ")
            )?,
            Statement::If(condition, code, elseif, r#else) => {
                write!(f, "if ( {} ) {}", condition, code)?;
                for (cond, c) in elseif {
                    write!(f, "else if ( {} ) {}", cond, c)?;
                }

                if let Some(e) = r#else {
                    write!(f, "else {}", e)?;
                }
            }
        };
        Ok(())
    }
}

pub struct NameRegister {
    names: Vec<String>,
    
}
