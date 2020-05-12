use super::{Backend, Result};
use crate::il::Block;

pub struct JavascriptBackend;

impl Backend for JavascriptBackend {
    fn compile(b: Block) -> Result {
        Result::File(String::from("out.js"), String::new())
    }
}