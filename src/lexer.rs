use crate::ast::Expr;

pub fn tokenize(input: &str) -> Vec<String> {
    input
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

pub fn parse(tokens: &mut Vec<String>) -> Expr {
    if tokens.is_empty() {
        panic!("Unexpected EOF");
    }

    let token = tokens.remove(0);
    match token.as_str() {
        "(" => {
            let mut list = Vec::new();
            while tokens[0] != ")" {
                list.push(parse(tokens));
            }
            tokens.remove(0);
            Expr::List(list)
        }
        ")" => panic!("Unexpected ')'"),
        _ => {
            if let Ok(num) = token.parse::<f64>() {
                Expr::Number(num)
            } else {
                Expr::Symbol(token)
            }
        }
    }
}
