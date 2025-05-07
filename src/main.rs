use std::io::Write;

use scheme_parser::{
    env::Env,
    eval::eval,
    lexer::{parse, tokenize},
};

fn main() {
    let mut env = Env::new();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            break;
        }
        let mut tokens = tokenize(&input);
        let expr = parse(&mut tokens);
        let result = eval(&expr, &mut env);
        if result.is_err() {
            println!("Error: {:#?}", result.as_ref().err().unwrap());
        } else {
            println!("{}", result.unwrap());
        }
    }
}
