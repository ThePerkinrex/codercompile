use codercompile::backends::{js::Version, Backend, JavascriptBackend, NameRegistry};
use codercompile::il::{Block, Statement, Type, Value};

fn main() {
    let mut b = Block::new();
    b.add(Statement::Val(Value::FnCall(
        "console.log".to_string(),
        vec![Value::Int(69)],
    )));
    let mut ifcode = Block::new();
    ifcode.add(Statement::VarDeclare(
        "t".to_string(),
        Type::Int,
        Value::Int(80),
    ));
    b.add(Statement::If(Value::Bool(true), ifcode.clone(), vec![], Some(ifcode)));
    println!("Hello, \n{}", b);
    let mut backend = JavascriptBackend::new(Version::ES6, "out.js");
    let mut name_reg = NameRegistry::new();
    name_reg.add("console.log", Type::Function(vec![Type::Int], Box::new(Type::Null)));
    let o = backend.compile(&b, &mut name_reg);
    println!("{:?}", o);
    // println!("HHHH {}", Value::Type(Type::Array(Box::new(Type::Array(Box::new(Type::Int))))));
}
