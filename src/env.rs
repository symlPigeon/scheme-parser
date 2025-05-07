use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::Expr,
    eval::EvalError,
    value::{BuiltinFunc, Value},
};

#[derive(Debug, Clone, Default)]
pub struct Env {
    pub vars: HashMap<String, Value>,
    pub parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new() -> Self {
        let mut env = Env {
            vars: HashMap::new(),
            parent: None,
        };
        env.define_builtin();
        env
    }

    pub fn new_child(&self) -> Env {
        Env {
            vars: HashMap::new(),
            parent: Some(Rc::new(RefCell::new(self.clone()))),
        }
    }

    pub fn define(&mut self, name: &str, value: Value) {
        self.vars.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        match self.vars.get(name) {
            Some(val) => Some(val.clone()),
            None => {
                if let Some(parent) = &self.parent {
                    parent.borrow().get(name)
                } else {
                    None
                }
            }
        }
    }

    fn define_builtin(&mut self) {
        self.define(
            "+",
            Value::BuiltinFunction(BuiltinFunc {
                func: |args, expr| {
                    let sum = args
                        .iter()
                        .map(|val| match val {
                            Value::Number(num) => Ok(*num),
                            _ => Err(EvalError::TypeError {
                                expected: "Number".to_string(),
                                found: val.clone(),
                                in_expr: expr.clone(),
                            }),
                        })
                        .collect::<Result<Vec<f64>, EvalError>>()?
                        .iter()
                        .sum();
                    Ok(Value::Number(sum))
                },
                name: "+".to_string(),
            }),
        );
        self.define(
            "-",
            Value::BuiltinFunction(BuiltinFunc {
                func: |args, expr| {
                    if args.len() == 1 {
                        if let Value::Number(val) = args[0] {
                            return Ok(Value::Number(-val));
                        } else {
                            return Err(Box::new(EvalError::TypeError {
                                expected: "Number".to_string(),
                                found: args[0].clone(),
                                in_expr: expr.clone(),
                            }));
                        }
                    }
                    let first = if let Value::Number(val) = args[0] {
                        val
                    } else {
                        return Err(Box::new(EvalError::TypeError {
                            expected: "Number".to_string(),
                            found: args[0].clone(),
                            in_expr: expr.clone(),
                        }));
                    };
                    let rest = &args[1..];
                    let sum = rest
                        .iter()
                        .map(|val| match val {
                            Value::Number(num) => Ok(*num),
                            _ => Err(EvalError::TypeError {
                                expected: "Number".to_string(),
                                found: val.clone(),
                                in_expr: expr.clone(),
                            }),
                        })
                        .collect::<Result<Vec<f64>, EvalError>>()?
                        .iter()
                        .sum::<f64>();
                    Ok(Value::Number(first - sum))
                },
                name: "-".to_string(),
            }),
        );
        self.define(
            "*",
            Value::BuiltinFunction(BuiltinFunc {
                func: |args, expr| {
                    if args.len() < 2 {
                        return Err(Box::new(EvalError::InvalidSyntax {
                            expr,
                            desc: "Expected at least 2 arguments".to_string(),
                        }));
                    }
                    let product = args
                        .iter()
                        .map(|val| match val {
                            Value::Number(num) => Ok(*num),
                            _ => Err(EvalError::TypeError {
                                expected: "Number".to_string(),
                                found: val.clone(),
                                in_expr: expr.clone(),
                            }),
                        })
                        .collect::<Result<Vec<f64>, EvalError>>()?
                        .iter()
                        .product();
                    Ok(Value::Number(product))
                },
                name: "*".to_string(),
            }),
        );
        self.define(
            "/",
            Value::BuiltinFunction(BuiltinFunc {
                func: |args, expr| {
                    if args.len() == 1 {
                        if let Value::Number(val) = args[0] {
                            return Ok(Value::Number(1.0 / val));
                        } else {
                            return Err(Box::new(EvalError::TypeError {
                                expected: "Number".to_string(),
                                found: args[0].clone(),
                                in_expr: expr.clone(),
                            }));
                        }
                    }
                    let first = if let Value::Number(val) = args[0] {
                        val
                    } else {
                        return Err(Box::new(EvalError::TypeError {
                            expected: "Number".to_string(),
                            found: args[0].clone(),
                            in_expr: expr.clone(),
                        }));
                    };
                    let rest = &args[1..];
                    let product = rest
                        .iter()
                        .map(|val| match val {
                            Value::Number(num) => Ok(*num),
                            _ => Err(EvalError::TypeError {
                                expected: "Number".to_string(),
                                found: val.clone(),
                                in_expr: expr.clone(),
                            }),
                        })
                        .collect::<Result<Vec<f64>, EvalError>>()?
                        .iter()
                        .product::<f64>();
                    Ok(Value::Number(first / product))
                },
                name: "/".to_string(),
            }),
        );
        self.define(
            "<",
            Value::BuiltinFunction(BuiltinFunc {
                func: |args, expr| {
                    if args.len() != 2 {
                        return Err(Box::new(EvalError::InvalidSyntax {
                            expr,
                            desc: "Expected 2 arguments".to_string(),
                        }));
                    }
                    let first = if let Value::Number(val) = args[0] {
                        val
                    } else {
                        return Err(Box::new(EvalError::TypeError {
                            expected: "Number".to_string(),
                            found: args[0].clone(),
                            in_expr: expr.clone(),
                        }));
                    };
                    let second = if let Value::Number(val) = args[1] {
                        val
                    } else {
                        return Err(Box::new(EvalError::TypeError {
                            expected: "Number".to_string(),
                            found: args[1].clone(),
                            in_expr: expr.clone(),
                        }));
                    };
                    Ok(Value::Bool(first < second))
                },
                name: "<".to_string(),
            }),
        );
        self.define(
            "<=",
            Value::BuiltinFunction(BuiltinFunc {
                func: |args, expr| {
                    if args.len() != 2 {
                        return Err(Box::new(EvalError::InvalidSyntax {
                            expr,
                            desc: "Expected 2 arguments".to_string(),
                        }));
                    }
                    let first = if let Value::Number(val) = args[0] {
                        val
                    } else {
                        return Err(Box::new(EvalError::TypeError {
                            expected: "Number".to_string(),
                            found: args[0].clone(),
                            in_expr: expr.clone(),
                        }));
                    };
                    let second = if let Value::Number(val) = args[1] {
                        val
                    } else {
                        return Err(Box::new(EvalError::TypeError {
                            expected: "Number".to_string(),
                            found: args[1].clone(),
                            in_expr: expr.clone(),
                        }));
                    };
                    Ok(Value::Bool(first <= second))
                },
                name: "<=".to_string(),
            }),
        );
        self.define(
            ">",
            Value::BuiltinFunction(BuiltinFunc {
                func: |args, expr| {
                    if args.len() != 2 {
                        return Err(Box::new(EvalError::InvalidSyntax {
                            expr,
                            desc: "Expected 2 arguments".to_string(),
                        }));
                    }
                    let first = if let Value::Number(val) = args[0] {
                        val
                    } else {
                        return Err(Box::new(EvalError::TypeError {
                            expected: "Number".to_string(),
                            found: args[0].clone(),
                            in_expr: expr.clone(),
                        }));
                    };
                    let second = if let Value::Number(val) = args[1] {
                        val
                    } else {
                        return Err(Box::new(EvalError::TypeError {
                            expected: "Number".to_string(),
                            found: args[1].clone(),
                            in_expr: expr.clone(),
                        }));
                    };
                    Ok(Value::Bool(first > second))
                },
                name: ">".to_string(),
            }),
        );
        self.define(
            ">=",
            Value::BuiltinFunction(BuiltinFunc {
                func: |args, expr| {
                    if args.len() != 2 {
                        return Err(Box::new(EvalError::InvalidSyntax {
                            expr,
                            desc: "Expected 2 arguments".to_string(),
                        }));
                    }
                    let first = if let Value::Number(val) = args[0] {
                        val
                    } else {
                        return Err(Box::new(EvalError::TypeError {
                            expected: "Number".to_string(),
                            found: args[0].clone(),
                            in_expr: expr.clone(),
                        }));
                    };
                    let second = if let Value::Number(val) = args[1] {
                        val
                    } else {
                        return Err(Box::new(EvalError::TypeError {
                            expected: "Number".to_string(),
                            found: args[1].clone(),
                            in_expr: expr.clone(),
                        }));
                    };
                    Ok(Value::Bool(first >= second))
                },
                name: ">=".to_string(),
            }),
        );
        self.define(
            "=",
            Value::BuiltinFunction(BuiltinFunc {
                func: |args, expr| {
                    if args.len() != 2 {
                        return Err(Box::new(EvalError::InvalidSyntax {
                            expr,
                            desc: "Expected 2 arguments".to_string(),
                        }));
                    }
                    let first = if let Value::Number(val) = args[0] {
                        val
                    } else {
                        return Err(Box::new(EvalError::TypeError {
                            expected: "Number".to_string(),
                            found: args[0].clone(),
                            in_expr: expr.clone(),
                        }));
                    };
                    let second = if let Value::Number(val) = args[1] {
                        val
                    } else {
                        return Err(Box::new(EvalError::TypeError {
                            expected: "Number".to_string(),
                            found: args[1].clone(),
                            in_expr: expr.clone(),
                        }));
                    };
                    Ok(Value::Bool(first == second))
                },
                name: "=".to_string(),
            }),
        );
        self.define(
            "!=",
            Value::BuiltinFunction(BuiltinFunc {
                func: |args, expr| {
                    if args.len() != 2 {
                        return Err(Box::new(EvalError::InvalidSyntax {
                            expr,
                            desc: "Expected 2 arguments".to_string(),
                        }));
                    }
                    let first = if let Value::Number(val) = args[0] {
                        val
                    } else {
                        return Err(Box::new(EvalError::TypeError {
                            expected: "Number".to_string(),
                            found: args[0].clone(),
                            in_expr: expr.clone(),
                        }));
                    };
                    let second = if let Value::Number(val) = args[1] {
                        val
                    } else {
                        return Err(Box::new(EvalError::TypeError {
                            expected: "Number".to_string(),
                            found: args[1].clone(),
                            in_expr: expr.clone(),
                        }));
                    };
                    Ok(Value::Bool(first != second))
                },
                name: "!=".to_string(),
            }),
        );
    }
}
