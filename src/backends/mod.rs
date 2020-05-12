use crate::il::Block;

mod js;
pub use js::JavascriptBackend;

pub trait Backend {
    fn compile(b: Block) -> Result;
}

pub enum Result {
    File(String, String),
    Folder(String, Box<Result>),

    // Filename, line, column, errorName, errorTrace
    Err(String, usize, usize, String, String)
}