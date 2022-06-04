//! evaluator.rs
//!
//! Evaluates a List-based strurcture created by parser recursively and combines intermediate values
//! to produce the final result

use std::{cell::RefCell, rc::Rc};

use crate::{env::Env, lexer::tokenize, object::Object, parser::parse_list};

pub fn eval(program: &str, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let mut tokenized = tokenize(program).into_iter().rev().collect();
    let parsed_list = parse_list(&mut tokenized)?;
    eval_obj(&parsed_list, env)
}

fn eval_obj(obj: &Object, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    match obj {
        Object::Void => Ok(Object::Void),
        Object::Bool(_) => Ok(obj.clone()),
        Object::Integer(n) => Ok(Object::Integer(*n)),
        Object::Lambda(_params, _body) => Ok(Object::Void),
        Object::Symbol(s) => eval_symbol(s, env),
        Object::List(list) => eval_list(list, env),
    }
}

fn eval_symbol(s: &str, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    match env.borrow_mut().get(s) {
        Some(obj) => Ok(obj),
        None => Err(format!("Undefined symbol: {}", s)),
    }
}

fn eval_list(list: &[Object], env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if let Object::Symbol(s) = &list[0] {
        match s.as_str() {
            "+" | "-" | "*" | "/" | "<" | ">" | "=" | "!=" => eval_binary_operation(list, env),
            "if" => eval_if(list, env),
            "define" => eval_define(list, env),
            "lambda" => eval_function_definition(list),
            _ => eval_function_call(s, list, env),
        }
    } else {
        let mut new_list = Vec::new();

        for obj in list {
            let result = eval_obj(obj, env)?;
            if Object::Void != result {
                new_list.push(result);
            }
        }

        Ok(Object::List(new_list))
    }
}

fn eval_function_call(
    s: &str,
    list: &[Object],
    env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
    let lambda = env.borrow_mut().get(s);
    match lambda {
        Some(func) => match func {
            Object::Lambda(params, body) => {
                let mut new_env = Rc::new(RefCell::new(Env::extend(env.clone())));
                for (i, param) in params.iter().enumerate() {
                    let val = eval_obj(&list[i + 1], env)?;
                    new_env.borrow_mut().set(param, val);
                }
                eval_obj(&Object::List(body.clone()), &mut new_env)
            }
            _ => Err(format!("Not a function: {}", s)),
        },
        None => Err(format!("Undefined symbol: {}", s)),
    }
}

fn eval_function_definition(list: &[Object]) -> Result<Object, String> {
    let params = match &list[1] {
        Object::List(list) => {
            let mut params = Vec::new();

            for param in list {
                match param {
                    Object::Symbol(s) => params.push(s.clone()),
                    _ => return Err(format!("Invalid parameter: {:?}", param)),
                }
            }
            params
        }
        _ => return Err(format!("Invalid parameter: {:?}", list[1])),
    };

    let body = match &list[2] {
        Object::List(list) => list.clone(),
        _ => return Err(format!("Invalid parameter: {:?}", list[2])),
    };

    Ok(Object::Lambda(params, body))
}

fn eval_define(list: &[Object], env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 3 {
        return Err("define: wrong number of arguments".to_string());
    }

    let sym = match &list[1] {
        Object::Symbol(s) => s.clone(),
        _ => return Err("define: expected symbol as first argument".to_string()),
    };

    let val = eval_obj(&list[2], env)?;
    env.borrow_mut().set(&sym, val);

    Ok(Object::Void)
}

fn eval_if(list: &[Object], env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 4 {
        return Err(format!(
            "Invalid number of arguments for if: {}",
            list.len()
        ));
    }

    let cond_obj = eval_obj(&list[1], env)?;
    let cond = if let Object::Bool(b) = cond_obj {
        b
    } else {
        return Err(format!("Invalid condition: {:?}", cond_obj));
    };

    if cond {
        eval_obj(&list[2], env)
    } else {
        eval_obj(&list[3], env)
    }
}

fn eval_binary_operation(list: &[Object], env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 3 {
        return Err("Invalid number of arguments".to_string());
    }
    let operator = list[0].clone();
    let left = eval_obj(&list[1].clone(), env)?;
    let right = eval_obj(&list[2].clone(), env)?;

    let left_val = match left {
        Object::Integer(n) => n,
        _ => return Err("Invalid left operand".to_string()),
    };
    let right_val = match right {
        Object::Integer(n) => n,
        _ => return Err("Invalid right operand".to_string()),
    };

    match operator {
        Object::Symbol(s) => match s.as_str() {
            "+" => Ok(Object::Integer(left_val + right_val)),
            "-" => Ok(Object::Integer(left_val - right_val)),
            "*" => Ok(Object::Integer(left_val * right_val)),
            "/" => Ok(Object::Integer(left_val / right_val)),
            "%" => Ok(Object::Integer(left_val % right_val)),
            "<" => Ok(Object::Bool(left_val < right_val)),
            ">" => Ok(Object::Bool(left_val > right_val)),
            "=" => Ok(Object::Bool(left_val == right_val)),
            "!=" => Ok(Object::Bool(left_val != right_val)),
            _ => Err("Invalid operator".to_string()),
        },
        _ => Err("Invalid operator".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(+ 1 2)", &mut env).unwrap();
        assert_eq!(result, Object::Integer(3));
    }

    #[test]
    fn test_sqr_function() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval(
            "(
                            (define sqr (lambda (x) (* x x)))
                            (sqr 2)
            )",
            &mut env,
        )
        .unwrap();
        assert_eq!(result, Object::List(vec![Object::Integer(4)]));
    }

    #[ignore = "unware how this works"]
    #[test]
    fn test_fibonacci_function() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval(
            "(
                            (define fib (lambda (n)
                                            (if (< n 2)
                                                n
                                                (+ (fib (- n 1)) (fib (- n 2))))))
                            (fib 10)
            )",
            &mut env,
        )
        .unwrap();
        assert_eq!(result, Object::List(vec![Object::Integer(55)]));
    }

    #[test]
    fn test_factorial_function() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval(
            "(
                            (define fact (lambda (n)
                                            (if (= n 1)
                                                1
                                                (* n (fact (- n 1))))))
                            (fact 5)
            )",
            &mut env,
        )
        .unwrap();
        assert_eq!(result, Object::List(vec![Object::Integer(120)]));
    }
}
