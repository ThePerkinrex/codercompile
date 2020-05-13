use crate::il::{Block, Type};
use std::collections::HashMap;

pub mod js;
pub use js::JavascriptBackend;

pub trait Backend {
    fn compile(&mut self, b: &Block, ctx: &mut NameRegistry) -> Result<String, ()>;
}

/*#[derive(Debug)]
pub enum Result {
    File(String, String),
    Folder(String, Box<Result>),

    // Filename, line, column, errorName, errorTrace
    Err(String, usize, usize, String, String),
}*/

#[derive(Debug, Clone)]
pub struct NameRegistry {
    names: HashMap<String, Type>,
}

impl NameRegistry {
    pub fn new() -> NameRegistry {
        NameRegistry {
            names: HashMap::new(),
        }
    }

    pub fn add<T: ToString>(&mut self, name: T, r#type: Type) -> NameRegistryResult {
        if let Some(t) = self.names.insert(name.to_string(), r#type) {
            self.names.insert(name.to_string(), t).unwrap();
            return NameRegistryResult::NamePresentError(name.to_string());
        }
        NameRegistryResult::Ok
    }

    pub fn get<T: ToString>(&self, name: T) -> NameRegistryResult {
        self.names
            .get(&name.to_string())
            .and_then(|t| Some(NameRegistryResult::Type(t.clone())))
            .unwrap_or(NameRegistryResult::NameNotFoundError(name.to_string()))
    }

    pub fn is_defined<T: ToString>(&self, name: T) -> bool {
        self.names.contains_key(&name.to_string())
    }
}

#[derive(Debug)]
pub enum NameRegistryResult {
    Ok,
    Type(Type),
    NamePresentError(String),
    NameNotFoundError(String),
}
