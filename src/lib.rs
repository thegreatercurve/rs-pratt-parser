mod lexer;
mod parser;
mod token;

use std::io::stdin;

use lexer::Lexer;
use parser::Parser;

fn main() {
    println!("Expression parser REPL!");

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let result = evaluate(&input);

        match result {
            Ok(value) => println!("Result: {}", value),
            Err(error) => eprintln!("Error: {}", error),
        }
    }
}

fn evaluate(input: &str) -> Result<i32, String> {
    let lexer = Parser::new(Lexer::new(&input));

    match input.parse::<i32>() {
        Ok(value) => Ok(value),
        Err(_) => Err(String::from("Invalid input")),
    }
}
