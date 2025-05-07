use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::value::Value;

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
            Value::BuiltinFunction(|args| {
                let sum = args
                    .iter()
                    .map(|val| match val {
                        Value::Number(num) => num,
                        _ => panic!("Expected a number"),
                    })
                    .sum();
                Value::Number(sum)
            }),
        );
        self.define(
            "-",
            Value::BuiltinFunction(|args| {
                if args.len() == 1 {
                    if let Value::Number(val) = args[0] {
                        return Value::Number(-val);
                    } else {
                        panic!("Expected a number");
                    }
                }
                let first = if let Value::Number(val) = args[0] {
                    val
                } else {
                    panic!("Expected a number");
                };
                let rest = &args[1..];
                let sum = rest
                    .iter()
                    .map(|val| match val {
                        Value::Number(num) => num,
                        _ => panic!("Expected a number"),
                    })
                    .sum::<f64>();
                Value::Number(first - sum)
            }),
        );
        self.define(
            "*",
            Value::BuiltinFunction(|args| {
                let product = args
                    .iter()
                    .map(|val| match val {
                        Value::Number(num) => num,
                        _ => panic!("Expected a number"),
                    })
                    .product();
                Value::Number(product)
            }),
        );
        self.define(
            "/",
            Value::BuiltinFunction(|args| {
                if args.len() == 1 {
                    if let Value::Number(val) = args[0] {
                        return Value::Number(1.0 / val);
                    } else {
                        panic!("Expected a number");
                    }
                }
                let first = if let Value::Number(val) = args[0] {
                    val
                } else {
                    panic!("Expected a number");
                };
                let rest = &args[1..];
                let product = rest
                    .iter()
                    .map(|val| match val {
                        Value::Number(num) => num,
                        _ => panic!("Expected a number"),
                    })
                    .product::<f64>();
                Value::Number(first / product)
            }),
        );
        self.define(
            "<",
            Value::BuiltinFunction(|args| {
                if args.len() != 2 {
                    panic!("Expected 2 arguments");
                }
                let first = if let Value::Number(val) = args[0] {
                    val
                } else {
                    panic!("Expected a number");
                };
                let second = if let Value::Number(val) = args[1] {
                    val
                } else {
                    panic!("Expected a number");
                };
                Value::Bool(first < second)
            }),
        );
        self.define(
            "<=",
            Value::BuiltinFunction(|args| {
                if args.len() != 2 {
                    panic!("Expected 2 arguments");
                }
                let first = if let Value::Number(val) = args[0] {
                    val
                } else {
                    panic!("Expected a number");
                };
                let second = if let Value::Number(val) = args[1] {
                    val
                } else {
                    panic!("Expected a number");
                };
                Value::Bool(first <= second)
            }),
        );
        self.define(
            ">",
            Value::BuiltinFunction(|args| {
                if args.len() != 2 {
                    panic!("Expected 2 arguments");
                }
                let first = if let Value::Number(val) = args[0] {
                    val
                } else {
                    panic!("Expected a number");
                };
                let second = if let Value::Number(val) = args[1] {
                    val
                } else {
                    panic!("Expected a number");
                };
                Value::Bool(first > second)
            }),
        );
        self.define(
            ">=",
            Value::BuiltinFunction(|args| {
                if args.len() != 2 {
                    panic!("Expected 2 arguments");
                }
                let first = if let Value::Number(val) = args[0] {
                    val
                } else {
                    panic!("Expected a number");
                };
                let second = if let Value::Number(val) = args[1] {
                    val
                } else {
                    panic!("Expected a number");
                };
                Value::Bool(first >= second)
            }),
        );
        self.define(
            "=",
            Value::BuiltinFunction(|args| {
                if args.len() != 2 {
                    panic!("Expected 2 arguments");
                }
                let first = if let Value::Number(val) = args[0] {
                    val
                } else {
                    panic!("Expected a number");
                };
                let second = if let Value::Number(val) = args[1] {
                    val
                } else {
                    panic!("Expected a number");
                };
                Value::Bool(first == second)
            }),
        );
        self.define(
            "!=",
            Value::BuiltinFunction(|args| {
                if args.len() != 2 {
                    panic!("Expected 2 arguments");
                }
                let first = if let Value::Number(val) = args[0] {
                    val
                } else {
                    panic!("Expected a number");
                };
                let second = if let Value::Number(val) = args[1] {
                    val
                } else {
                    panic!("Expected a number");
                };
                Value::Bool(first != second)
            }),
        );
    }
}
