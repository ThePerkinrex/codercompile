use codercompile::il::{Block, Value, Statement};

fn main() {
    let mut b = Block::new();
    b.add(Statement::FnCall(0, "fn".to_string(), vec![Value::Int(69)]));
    let mut ifcode = Block::new();
    ifcode.add(Statement::FnCall(0, "fn".to_string(), vec![Value::Int(69)]));
    b.add(Statement::If(Value::Bool(true), ifcode, vec![], None));
    println!("Hello, \n{}", b)
}