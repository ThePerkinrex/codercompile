use super::{Backend, NameRegistry, NameRegistryResult /*Result as BackendResult*/};
use crate::il::{Block, Statement, Value};

pub enum Version {
    ES6,
    ES5,
}

pub struct JavascriptBackend {
    v: Version,
    filename: String,
}

impl JavascriptBackend {
    pub fn new<T: ToString>(v: Version, file: T) -> JavascriptBackend {
        JavascriptBackend {
            v,
            filename: file.to_string(),
        }
    }
    fn compile_value(
        &mut self,
        v: &Value,
        ctx: &mut NameRegistry,
    ) -> Result<String, NameRegistryResult> {
        match v {
            Value::Int(i) => Ok(format!("{}", i)),
            Value::Uint(i) => Ok(format!("{}", i)),
            Value::Byte(b) => Ok(format!("{}", b)),
            Value::Char(c) => Ok(format!("{}", c)),
            Value::Bool(b) => Ok(format!("{}", if *b { "true" } else { "false" })),
            Value::FnCall(name, args) => {
                if ctx.is_defined(name) {
                    let mut string_args = Vec::new();
                    for arg in args.iter().map(|x| self.compile_value(x, ctx)) {
                        if let Ok(s) = arg {
                            string_args.push(s);
                        } else {
                            return Err(arg.unwrap_err());
                        }
                    }
                    Ok(format!("{}({})", name, string_args.join(",")))
                } else {
                    Err(NameRegistryResult::NameNotFoundError(name.clone()))
                }
            }
            _ => Ok(String::new()),
        }
    }
}

impl Backend for JavascriptBackend {
    fn compile(&mut self, b: &Block, ctx: &mut NameRegistry) -> Result<String, ()> {
        let mut res = String::new();
        for statement in b.iter() {
            match statement {
                Statement::Val(v) => {
                    let r = self.compile_value(v, ctx);
                    if let Ok(s) = r {
                        res += &format!("{};", s);
                    } else {
                        panic!("Compile error: {:?}", r.unwrap_err())
                    }
                }
                Statement::If(v, code, elifs, e) => {
                    {
                        let r = self.compile_value(v, ctx);
                        if let Ok(s) = r {
                            res += &format!(
                                "if({}){{{}}}",
                                s,
                                self.compile(code, &mut ctx.clone()).unwrap()
                            );
                        } else {
                            panic!("Compile error: {:?}", r.unwrap_err())
                        }
                    }
                    for (cond, c) in elifs {
                        let r = self.compile_value(cond, ctx);
                        if let Ok(s) = r {
                            res += &format!(
                                "elseif({}){{{}}}",
                                s,
                                self.compile(c, &mut ctx.clone()).unwrap()
                            );
                        } else {
                            panic!("Compile error: {:?}", r.unwrap_err())
                        }
                    }
                    if let Some(c) = e {
                        res += &format!("else{{{}}}", self.compile(c, &mut ctx.clone()).unwrap());
                    }
                }
                _ => (),
            };
        }
        Ok(res)
    }
}
