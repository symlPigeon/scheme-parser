use std::{cell::RefCell, fmt::Display, rc::Rc};

use colored::Colorize;

use crate::{ast::Expr, env::Env, eval::EvalError};

#[derive(Clone)]
pub struct UserFunction {
    pub params: Vec<String>,
    pub body: Expr,
    pub env: Rc<RefCell<Env>>,
    pub name: Option<String>,
}

#[derive(Clone, Debug)]
pub struct BuiltinFunc {
    pub name: String,
    pub func: fn(Vec<Value>, Expr) -> Result<Value, Box<EvalError>>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    BuiltinFunction(BuiltinFunc),
    Function(UserFunction),
    Nil
}

impl std::fmt::Debug for UserFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserFunction")
            .field("params", &self.params)
            .field("body", &self.body)
            .finish()
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n.to_string().blue()),
            Value::Bool(b) => write!(f, "{}", b.to_string().yellow()),
            Value::BuiltinFunction(BuiltinFunc{name: n,..}) => write!(f, "{}", n.red()),
            Value::Function(UserFunction{name: n,..}) => write!(f, "{}", n.as_ref().unwrap_or(&"".to_string()).red()),
            Value::Nil => write!(f, "{}", "nil".white().bold()),
        }
    }
}
