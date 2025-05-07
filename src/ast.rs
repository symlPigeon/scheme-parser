#[derive(Debug, Clone)]
pub enum Expr {
    Symbol(String),
    Number(f64),
    List(Vec<Expr>),
}
