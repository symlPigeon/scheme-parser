use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::Expr,
    env::Env,
    value::{BuiltinFunc, UserFunction, Value},
};

pub fn eval(expr: &Expr, env: &mut Env) -> Result<Value, Box<EvalError>> {
    match expr {
        Expr::Number(n) => Ok(Value::Number(*n)),
        Expr::Symbol(s) => Ok(env.get(s).ok_or(EvalError::UnboundSymbol(s.clone()))?),
        Expr::List(list) => {
            if list.is_empty() {
                return Ok(Value::Nil);
            }
            match &list[0] {
                Expr::Symbol(s) if s == "define" => {
                    if list.len() != 3 {
                        return Err(Box::new(EvalError::InvalidSyntax {
                            expr: expr.clone(),
                            desc: "define requires 2 arguments.".to_string(),
                        }));
                    }
                    match &list[1] {
                        Expr::Symbol(name) => {
                            let val = eval(&list[2], env)?;
                            env.define(name, val.clone());
                            Ok(val)
                        }
                        Expr::List(fn_decl) => {
                            if let Some(Expr::Symbol(name)) = fn_decl.first() {
                                let params: Vec<String> = fn_decl[1..]
                                    .iter()
                                    .map(|p| match p {
                                        Expr::Symbol(s) => Ok(s.clone()),
                                        _ => Err(EvalError::InvalidSyntax {
                                            expr: expr.clone(),
                                            desc: "Function parameters must be symbols."
                                                .to_string(),
                                        }),
                                    })
                                    .collect::<Result<Vec<_>, EvalError>>()?;
                                let body = list[2].clone();
                                // manually evaluate the function body
                                let func_env = Rc::new(RefCell::new(env.clone()));
                                let val = Value::Function(UserFunction {
                                    params: params.clone(),
                                    body: body.clone(),
                                    env: Rc::clone(&func_env),
                                    name: Some(name.clone()),
                                });
                                func_env.borrow_mut().vars.insert(name.clone(), val.clone());

                                env.define(name, val.clone());
                                Ok(val)
                            } else {
                                Err(Box::new(EvalError::InvalidSyntax {
                                    expr: expr.clone(),
                                    desc: "Function name must be a symbol.".to_string(),
                                }))
                            }
                        }
                        _ => Err(Box::new(EvalError::InvalidSyntax {
                            expr: expr.clone(),
                            desc: "define requires a symbol or a list".to_string(),
                        })),
                    }
                }
                Expr::Symbol(s) if s == "lambda" => {
                    if list.len() != 3 {
                        return Err(Box::new(EvalError::InvalidSyntax {
                            expr: expr.clone(),
                            desc: "lambda requires 2 arguments".to_string(),
                        }));
                    }
                    let params = match &list[1] {
                        Expr::List(l) => l
                            .iter()
                            .map(|e| match e {
                                Expr::Symbol(s) => Ok(s.clone()),
                                _ => Err(EvalError::InvalidSyntax {
                                    expr: expr.clone(),
                                    desc: "lambda parameters must be symbols".to_string(),
                                }),
                            })
                            .collect::<Result<Vec<_>, EvalError>>()?,
                        _ => {
                            return Err(Box::new(EvalError::InvalidSyntax {
                                expr: expr.clone(),
                                desc: "lambda parameters must be a list".to_string(),
                            }));
                        }
                    };
                    let body = list[2].clone();
                    Ok(Value::Function(UserFunction {
                        params,
                        body,
                        env: Rc::new(RefCell::new(env.clone())),
                        name: None,
                    }))
                }
                Expr::Symbol(s) if s == "and" => {
                    if list.len() < 3 {
                        return Err(Box::new(EvalError::InvalidSyntax {
                            expr: expr.clone(),
                            desc: "and requires at least 2 arguments".to_string(),
                        }));
                    }
                    for arg in &list[1..] {
                        let val = eval(arg, env)?;
                        match val {
                            Value::Bool(true) => {
                                continue;
                            }
                            Value::Bool(false) => {
                                return Ok(Value::Bool(false));
                            }
                            _ => {
                                return Err(Box::new(EvalError::InvalidSyntax {
                                    expr: expr.clone(),
                                    desc: "and requires boolean arguments".to_string(),
                                }));
                            }
                        }
                    }
                    Ok(Value::Bool(true))
                }
                Expr::Symbol(s) if s == "or" => {
                    if list.len() < 3 {
                        return Err(Box::new(EvalError::InvalidSyntax {
                            expr: expr.clone(),
                            desc: "or requires at least 2 arguments".to_string(),
                        }));
                    }
                    for arg in &list[1..] {
                        let val = eval(arg, env)?;
                        match val {
                            Value::Bool(true) => {
                                return Ok(Value::Bool(true));
                            }
                            Value::Bool(false) => {
                                continue;
                            }
                            _ => {
                                return Err(Box::new(EvalError::InvalidSyntax {
                                    expr: expr.clone(),
                                    desc: "or requires boolean arguments".to_string(),
                                }));
                            }
                        }
                    }
                    Ok(Value::Bool(false))
                }
                Expr::Symbol(s) if s == "not" => {
                    if list.len() != 2 {
                        return Err(Box::new(EvalError::InvalidSyntax {
                            expr: expr.clone(),
                            desc: "not requires 1 argument".to_string(),
                        }));
                    }
                    let val = eval(&list[1], env)?;
                    match val {
                        Value::Bool(b) => Ok(Value::Bool(!b)),
                        _ => Err(Box::new(EvalError::InvalidSyntax {
                            expr: expr.clone(),
                            desc: "not requires a boolean argument".to_string(),
                        })),
                    }
                }
                Expr::Symbol(s) if s == "if" => {
                    if list.len() != 4 {
                        return Err(Box::new(EvalError::InvalidSyntax {
                            expr: expr.clone(),
                            desc: "if requires 3 arguments".to_string(),
                        }));
                    }
                    let cond = eval(&list[1], env)?;
                    match cond {
                        Value::Bool(true) => eval(&list[2], env),
                        Value::Bool(false) => eval(&list[3], env),
                        _ => Err(Box::new(EvalError::InvalidSyntax {
                            expr: expr.clone(),
                            desc: "if condition must be a boolean".to_string(),
                        })),
                    }
                }
                Expr::Symbol(s) if s == "cond" => {
                    if list.len() < 3 {
                        return Err(Box::new(EvalError::InvalidSyntax {
                            expr: expr.clone(),
                            desc: "cond requires at least 2 arguments".to_string(),
                        }));
                    }
                    for pair in &list[1..] {
                        if let Expr::List(l) = pair {
                            if l.len() != 2 {
                                return Err(Box::new(EvalError::InvalidSyntax {
                                    expr: expr.clone(),
                                    desc: "cond requires a list of pairs".to_string(),
                                }));
                            }
                            let (cond, val) = (&l[0], &l[1]);
                            if let Expr::Symbol(s) = cond {
                                if s == "else" {
                                    return eval(val, env);
                                }
                            }
                            let cond_val = eval(cond, env)?;
                            if let Value::Bool(flag) = cond_val {
                                if flag {
                                    return eval(val, env);
                                }
                            } else {
                                return Err(Box::new(EvalError::TypeError {
                                    expected: "bool".to_string(),
                                    found: cond_val,
                                    in_expr: cond.clone(),
                                }));
                            }
                        } else {
                            return Err(Box::new(EvalError::InvalidSyntax {
                                expr: expr.clone(),
                                desc: "cond requires a list of pairs".to_string(),
                            }));
                        }
                    }
                    Err(Box::new(EvalError::InvalidSyntax {
                        expr: expr.clone(),
                        desc: "cond requires at least one else clause".to_string(),
                    }))
                }
                func_expr => {
                    let func = eval(func_expr, env)?;
                    let args: Vec<Value> = list[1..]
                        .iter()
                        .map(|arg| eval(arg, env))
                        .collect::<Result<Vec<Value>, Box<EvalError>>>()?;
                    match func {
                        Value::BuiltinFunction(BuiltinFunc { func: f, .. }) => {
                            f(args, expr.clone())
                        }
                        Value::Function(UserFunction {
                            params,
                            body,
                            env: func_env,
                            name: _,
                        }) => {
                            if params.len() != args.len() {
                                return Err(Box::new(EvalError::ArityMismatch {
                                    expected: params.len(),
                                    found: args.len(),
                                    in_expr: expr.clone(),
                                }));
                            }
                            let mut local_env = func_env.borrow().clone();
                            for (name, val) in params.iter().zip(args.into_iter()) {
                                local_env.define(name, val);
                            }

                            eval(&body, &mut local_env)
                        }
                        _ => Err(Box::new(EvalError::TypeError {
                            expected: "function".to_string(),
                            found: func,
                            in_expr: func_expr.clone(),
                        })),
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum EvalError {
    UnboundSymbol(String),
    InvalidSyntax {
        expr: Expr,
        desc: String,
    },
    TypeError {
        expected: String,
        found: Value,
        in_expr: Expr,
    },
    ArityMismatch {
        expected: usize,
        found: usize,
        in_expr: Expr,
    },
    OtherError(String),
}
