use std::fmt::Display;

use colored::Colorize;

#[derive(Debug, Clone)]
pub enum Expr {
    Symbol(String),
    Number(f64),
    List(Vec<Expr>)
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Symbol(s) => write!(f, "{}", s.green()),
            Expr::Number(n) => write!(f, "{}", n.to_string().blue()),
            Expr::List(l) => {
                write!(f, "(")?;
                for (i, expr) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{expr}")?;
                }
                write!(f, ")")?;
                Ok(())
            }
        }
    }
}