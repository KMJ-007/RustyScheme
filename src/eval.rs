/* heart of program */

use crate::env::*;
use crate::parser::{parse, Object};
use std::cell::RefCell;
use std::rc::Rc;

pub fn hello_karan() {
    println!("hello Karan from eval");
}

fn eval_function_call(
    s: &str,
    list: &Vec<Object>,
    env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
    let lambda = env.borrow_mut().get(s);
    if lambda.is_none() {
        return Err(format!("Unbound symbol:{}", s));
    }

    let func = lambda.unwrap();
    match func {
        Object::Lambda(params, body) => {
            // this can be made much more simpler ?
            let mut new_env = Rc::new(RefCell::new(Env::extend(env.clone())));
            for (i, param) in params.iter().enumerate() {
                let val = eval_obj(&list[i + 1], env)?;
                new_env.borrow_mut().set(&param, val)
            }
            return eval_obj(&Object::List(body), &mut new_env);
        }
        _ => return Err(format!("not lambda: {:?}", s)),
    }
}

fn eval_lambda(list: &Vec<Object>) -> Result<Object, String> {
    let params = match &list[1] {
        Object::List(list) => {
            let mut params = Vec::new();
            for param in list {
                match param {
                    Object::Symbol(s) => params.push(s.clone()),
                    _ => return Err(format!("Invalid lambda parameter")),
                }
            }
            params
        }
        _ => return Err(format!("Invalid lambda")),
    };

    let body = match &list[2] {
        Object::List(list) => list.clone(),
        _ => return Err(format!("Invalid lambda")),
    };

    Ok(Object::Lambda(params, body))
}

fn eval_if(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 4 {
        return Err(format!("Insufficient number of arguments for if"));
    }
    let cond_obj = eval_obj(&list[1], env)?;
    let cond = match cond_obj {
        Object::Bool(b) => b,
        _ => return Err(format!("Condition must be a boolean")),
    };

    if cond == true {
        return eval_obj(&list[2], env);
    } else {
        return eval_obj(&list[3], env);
    }

    Ok(Object::Void)
}

fn eval_define(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 3 {
        return Err(format!("Insufficient number of arguments for define"));
    }
    let sym = match &list[1] {
        Object::Symbol(s) => s.clone(),
        _ => return Err(format!("Invalid define")),
    };
    let val = eval_obj(&list[2], env)?;
    env.borrow_mut().set(&sym, val);
    Ok(Object::Void)
}

fn eval_binary_op(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 3 {
        return Err(format!("Insufficient number of arguments"));
    }
    let operator = list[0].clone();
    let left = eval_obj(&list[1], env)?;
    let right = eval_obj(&list[2], env)?;
    let left_val = match left {
        Object::Integer(n) => n,
        _ => return Err(format!("Left operand must be an Int, found {:?}", left)),
    };
    let right_value = match right {
        Object::Integer(n) => n,
        _ => return Err(format!("right operand must be an Int, found {:?}", right)),
    };

    match operator {
        Object::Symbol(s) => match s.as_str() {
            "+" => Ok(Object::Integer(left_val + right_value)),
            "-" => Ok(Object::Integer(left_val - right_value)),
            "*" => Ok(Object::Integer(left_val * right_value)),
            "/" => Ok(Object::Integer(left_val / right_value)),
            "<" => Ok(Object::Bool(left_val < right_value)),
            ">" => Ok(Object::Bool(left_val > right_value)),
            "=" => Ok(Object::Bool(left_val == right_value)),
            "!=" => Ok(Object::Bool(left_val != right_value)),
            _ => Err(format!("Invalid Operator: {:?}", s)),
        },
        _ => Err(format!("operator must be a symbol")),
    }
}

fn eval_list(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let head = &list[0];
    match head {
        Object::Symbol(s) => match s.as_str() {
            "+" | "-" | "*" | "/" | "<" | ">" | "=" | "!=" => {
                return eval_binary_op(&list, env);
            }
            "define" => eval_define(&list, env),
            "if" => eval_if(&list, env),
            "lambda" => eval_lambda(&list),
            _ => eval_function_call(&s, &list, env),
        },
        _ => {
            let mut new_list = Vec::new();
            for obj in list {
                let result = eval_obj(obj, env)?;
                match result {
                    Object::Void => {}
                    _ => new_list.push(result),
                }
            }
            Ok(Object::List(new_list))
        }
    }
}

fn eval_symbol(s: &str, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let val = env.borrow_mut().get(s);
    if val.is_none() {
        return Err(format!("Unbound symbol: {}", s));
    }
    Ok(val.unwrap().clone())
}

fn eval_obj(obj: &Object, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    match obj {
        Object::Void => Ok(Object::Void),
        Object::Bool(_) => Ok(obj.clone()),
        Object::Integer(n) => Ok(Object::Integer(*n)),
        // todo
        Object::Lambda(_params, _body) => Ok(Object::Void),
        Object::Symbol(s) => eval_symbol(s, env),
        Object::List(list) => eval_list(list, env),
    }
}

pub fn eval(program: &str, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let parsed_list = parse(program);
    if parsed_list.is_err() {
        return Err(format!("{:?}", parsed_list.err().unwrap()));
    }
    eval_obj(&parsed_list.unwrap(), env)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(+ 1 2)", &mut env).unwrap();
        assert_eq!(result, Object::Integer(3));
    }

    #[test]
    fn test_area_of_a_circle() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "(
                        (define r 10)
                        (define pi 314)
                        (* pi (* r r))
                      )";
        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Integer((314 * 10 * 10) as i64)])
        );
    }

    #[test]
    fn test_sqr_function() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "(
                        (define sqr (lambda (r) (* r r))) 
                        (sqr 10)
                       )";
        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Integer((10 * 10) as i64)])
        );
    }

    #[test]
    fn test_fibonaci() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (
                (define fib (lambda (n) (if (< n 2) 1 (+ (fib (- n 1)) (fib (- n 2))))))
                (fib 10)
            )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::List(vec![Object::Integer((89) as i64)]));
    }

    #[test]
    fn test_factorial() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (
                (define fact (lambda (n) (if (< n 1) 1 (* n (fact (- n 1))))))
                (fact 5)
            )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::List(vec![Object::Integer((120) as i64)]));
    }

    #[test]
    fn test_circle_area_function() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (
                (define pi 314)
                (define r 10)
                (define sqr (lambda (r) (* r r)))
                (define area (lambda (r) (* pi (sqr r))))
                (area r)
            )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Integer((314 * 10 * 10) as i64)])
        );
    }
}
