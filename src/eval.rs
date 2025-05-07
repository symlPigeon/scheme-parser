use core::panic;
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::Expr,
    env::Env,
    value::{UserFunction, Value},
};

pub fn eval(expr: &Expr, env: &mut Env) -> Value {
    match expr {
        Expr::Number(n) => Value::Number(*n),
        Expr::Symbol(s) => env
            .get(s)
            .unwrap_or_else(|| panic!("Undefined variable: {}", s)),
        Expr::List(list) => {
            if list.is_empty() {
                panic!("Empty list");
            }
            match &list[0] {
                Expr::Symbol(s) if s == "define" => {
                    if list.len() != 3 {
                        panic!("Invalid Syntax: define requires 2 arguments");
                    }
                    match &list[1] {
                        Expr::Symbol(name) => {
                            let val = eval(&list[2], env);
                            env.define(name, val.clone());
                            val
                        }
                        Expr::List(fn_decl) => {
                            if let Some(Expr::Symbol(name)) = fn_decl.first() {
                                let params: Vec<String> = fn_decl[1..]
                                    .iter()
                                    .map(|p| match p {
                                        Expr::Symbol(s) => s.clone(),
                                        _ => panic!(
                                            "Invalid Syntax: function parameters must be symbols"
                                        ),
                                    })
                                    .collect();
                                let body = list[2].clone();
                                // manually evaluate the function body
                                let func_env = Rc::new(RefCell::new(env.clone()));
                                let val = Value::Function(UserFunction {
                                    params: params.clone(),
                                    body: body.clone(),
                                    env: Rc::clone(&func_env),
                                });
                                func_env.borrow_mut().vars.insert(name.clone(), val.clone());

                                env.define(name, val.clone());
                                val
                            } else {
                                panic!("Invalid Syntax: function name must be a symbol");
                            }
                        }
                        _ => {
                            panic!("Invalid Syntax: define requires a symbol or a list");
                        }
                    }
                }
                Expr::Symbol(s) if s == "lambda" => {
                    if list.len() != 3 {
                        panic!("Invalid Syntax: lambda requires 2 arguments");
                    }
                    let params = match &list[1] {
                        Expr::List(l) => l
                            .iter()
                            .map(|e| match e {
                                Expr::Symbol(s) => s.clone(),
                                _ => panic!("Invalid Syntax: lambda parameters must be symbols"),
                            })
                            .collect::<Vec<_>>(),
                        _ => panic!("Invalid Syntax: lambda parameters must be a list"),
                    };
                    let body = list[2].clone();
                    Value::Function(UserFunction {
                        params,
                        body,
                        env: Rc::new(RefCell::new(env.clone())),
                    })
                }
                Expr::Symbol(s) if s == "and" => {
                    if list.len() < 3 {
                        panic!("Invalid Syntax: and requires at least 2 arguments");
                    }
                    for arg in &list[1..] {
                        let val = eval(arg, env);
                        match val {
                            Value::Bool(true) => {
                                continue;
                            }
                            Value::Bool(false) => {
                                return Value::Bool(false);
                            }
                            _ => {
                                panic!("Invalid Syntax: and requires boolean arguments");
                            }
                        }
                    }
                    Value::Bool(true)
                }
                Expr::Symbol(s) if s == "or" => {
                    if list.len() < 3 {
                        panic!("Invalid Syntax: or requires at least 2 arguments");
                    }
                    for arg in &list[1..] {
                        let val = eval(arg, env);
                        match val {
                            Value::Bool(true) => {
                                return Value::Bool(true);
                            }
                            Value::Bool(false) => {
                                continue;
                            }
                            _ => {
                                panic!("Invalid Syntax: or requires boolean arguments");
                            }
                        }
                    }
                    Value::Bool(false)
                }
                Expr::Symbol(s) if s == "not" => {
                    if list.len() != 2 {
                        panic!("Invalid Syntax: not requires 1 argument");
                    }
                    let val = eval(&list[1], env);
                    match val {
                        Value::Bool(b) => Value::Bool(!b),
                        _ => panic!("Invalid Syntax: not requires a boolean argument"),
                    }
                }
                Expr::Symbol(s) if s == "if" => {
                    if list.len() != 4 {
                        panic!("Invalid Syntax: if requires 3 arguments");
                    }
                    let cond = eval(&list[1], env);
                    match cond {
                        Value::Bool(true) => eval(&list[2], env),
                        Value::Bool(false) => eval(&list[3], env),
                        _ => panic!("Invalid Syntax: if requires a boolean condition"),
                    }
                }
                Expr::Symbol(s) if s == "cond" => {
                    if list.len() < 3 {
                        panic!("Invalid Syntax: cond requires at least 2 arguments");
                    }
                    for pair in &list[1..] {
                        if let Expr::List(l) = pair {
                            if l.len() != 2 {
                                panic!("Invalid Syntax: cond pairs must have 2 elements");
                            }
                            let (cond, val) = (&l[0], &l[1]);
                            if let Expr::Symbol(s) = cond {
                                if s == "else" {
                                    return eval(val, env);
                                }
                            }
                            if let Value::Bool(flag) = eval(cond, env) {
                                if flag {
                                    return eval(val, env);
                                }
                            } else {
                                panic!("Invalid Syntax: cond requires boolean conditions");
                            }
                        } else {
                            panic!("Invalid Syntax: cond requires a list of pairs");
                        }
                    }
                    panic!("No true condition found in cond");
                }
                func_expr => {
                    let func = eval(func_expr, env);
                    let args: Vec<Value> = list[1..].iter().map(|arg| eval(arg, env)).collect();
                    match func {
                        Value::BuiltinFunction(f) => f(args),
                        Value::Function(UserFunction {
                            params,
                            body,
                            env: func_env,
                        }) => {
                            if params.len() != args.len() {
                                panic!("Invalid number of argumetns");
                            }
                            let mut local_env = func_env.borrow().clone();
                            for (name, val) in params.iter().zip(args.into_iter()) {
                                local_env.define(name, val);
                            }

                            eval(&body, &mut local_env)
                        }
                        _ => panic!("First element is not a function"),
                    }
                }
            }
        }
    }
}
