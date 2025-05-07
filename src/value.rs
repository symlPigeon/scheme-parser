use std::{cell::RefCell, rc::Rc};

use crate::{ast::Expr, env::Env};

#[derive(Clone)]
pub struct UserFunction {
    pub params: Vec<String>,
    pub body: Expr,
    pub env: Rc<RefCell<Env>>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    BuiltinFunction(fn(Vec<Value>) -> Value),
    Function(UserFunction),
}

impl std::fmt::Debug for UserFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserFunction")
            .field("params", &self.params)
            .field("body", &self.body)
            .finish()
    }
}
