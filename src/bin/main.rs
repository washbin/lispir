use std::{cell::RefCell, rc::Rc};

use linefeed::{Interface, ReadResult};
use lispir::{env::Env, evaluator::eval, object::Object};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = Interface::new("lispirλ ").expect("Failed to create linefeed reader");
    reader.set_prompt("lispirλ ").expect("Failed to set prompt");
    let mut env = Rc::new(RefCell::new(Env::new()));

    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        if input.eq("exit") {
            break;
        }
        let val = eval(input.as_ref(), &mut env)?;
        match val {
            Object::Void => (),
            Object::Integer(i) => println!("{}", i),
            Object::Bool(b) => println!("{}", b),
            Object::Symbol(s) => println!("{:?}", s),
            Object::Lambda(params, body) => {
                println!("Lambda(");
                for param in params {
                    println!("  {:?}", param);
                }
                println!(")");
                for expr in body {
                    println!("  {:?}", expr);
                }
            }
            _ => println!("{:?}", val),
        }
    }

    println!("Bye!");
    Ok(())
}
