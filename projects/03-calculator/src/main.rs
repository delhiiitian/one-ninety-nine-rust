// Project 3: Command-line calculator
//
//   cargo run -p calculator -- "2 * (3 + 4)"
//   cargo run -p calculator                      (interactive, type `quit`)

mod lexer;
mod parser;

use std::io::{self, BufRead, Write};

/// The whole pipeline: text -> tokens -> number.
pub fn evaluate(text: &str, ans: f64) -> Result<f64, String> {
    let tokens = lexer::tokenize(text)?;
    if tokens.is_empty() {
        return Err("nothing to calculate".to_string());
    }
    parser::Parser::new(&tokens, ans).parse()
}

/// Shows 7 instead of 7.0 and rounds away float noise like 0.30000000000000004.
fn format_number(value: f64) -> String {
    let rounded = (value * 1e10).round() / 1e10;
    if rounded.fract() == 0.0 && rounded.abs() < 1e15 {
        format!("{}", rounded as i64)
    } else {
        format!("{rounded}")
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if !args.is_empty() {
        match evaluate(&args.join(" "), 0.0) {
            Ok(v) => println!("{}", format_number(v)),
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
        return;
    }

    println!("Calculator. Try 2 * (3 + 4), sqrt(16), 2^10, ans / 2. Type quit to exit.");
    let mut ans = 0.0;
    let mut history: Vec<(String, f64)> = Vec::new();
    prompt();
    for line in io::stdin().lock().lines() {
        let line = line.expect("could not read input");
        let line = line.trim();
        match line {
            "" => {}
            "quit" | "exit" => break,
            "history" => {
                for (i, (expr, value)) in history.iter().enumerate() {
                    println!("{:>3}: {expr} = {}", i + 1, format_number(*value));
                }
            }
            _ => match evaluate(line, ans) {
                Ok(value) => {
                    println!("= {}", format_number(value));
                    ans = value;
                    history.push((line.to_string(), value));
                }
                Err(e) => println!("Error: {e}"),
            },
        }
        prompt();
    }
}

fn prompt() {
    print!("> ");
    io::stdout().flush().expect("could not flush stdout");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calc(text: &str) -> f64 {
        evaluate(text, 0.0).unwrap()
    }

    #[test]
    fn precedence() {
        assert_eq!(calc("2 + 3 * 4"), 14.0);
        assert_eq!(calc("(2 + 3) * 4"), 20.0);
        assert_eq!(calc("10 - 4 - 3"), 3.0); // left to right
        assert_eq!(calc("2 ^ 3 ^ 2"), 512.0); // right to left
        assert_eq!(calc("-2 ^ 2"), 4.0);
        assert_eq!(calc("17 % 5"), 2.0);
    }

    #[test]
    fn functions_and_ans() {
        assert_eq!(calc("sqrt(16) + abs(-2)"), 6.0);
        assert_eq!(evaluate("ans * 2", 21.0).unwrap(), 42.0);
        assert_eq!(format_number(calc("0.1 + 0.2")), "0.3");
    }

    #[test]
    fn errors() {
        assert_eq!(evaluate("1 / 0", 0.0), Err("division by zero".to_string()));
        assert_eq!(evaluate("(1 + 2", 0.0), Err("missing ')'".to_string()));
        assert!(evaluate("2 $ 3", 0.0).unwrap_err().contains("unexpected character"));
        assert!(evaluate("foo(1)", 0.0).unwrap_err().contains("unknown function"));
        assert!(evaluate("sqrt(-1)", 0.0).is_err());
    }
}
