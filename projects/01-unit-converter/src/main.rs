// Project 1: Unit converter
//
// Usage:
//   cargo run -p unit-converter -- 10 km mi
//   cargo run -p unit-converter -- list
//   cargo run -p unit-converter            (interactive mode)

mod units;

use std::io::{self, BufRead, Write};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() == 1 && args[0] == "list" {
        print_units();
    } else if args.len() == 3 {
        println!("{}", run_line(&args.join(" ")));
    } else if args.is_empty() {
        interactive();
    } else {
        eprintln!("Usage: unit-converter <value> <from> <to>   e.g. 10 km mi");
        eprintln!("       unit-converter list");
        std::process::exit(1);
    }
}

/// Reads one conversion per line until the user types "quit" or input ends.
fn interactive() {
    println!("Unit converter. Type e.g. `100 f c`, `list`, or `quit`.");
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().expect("could not flush stdout");
        let mut line = String::new();
        let bytes = stdin.lock().read_line(&mut line).expect("could not read input");
        let line = line.trim();
        if bytes == 0 || line == "quit" {
            println!("Bye!");
            break;
        }
        if line == "list" {
            print_units();
        } else if !line.is_empty() {
            println!("{}", run_line(line));
        }
    }
}

/// Turns a line like "10 km mi" into an answer or an error message.
fn run_line(line: &str) -> String {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 3 {
        return "Please type: <value> <from> <to>".to_string();
    }
    let value: f64 = match parts[0].parse() {
        Ok(v) => v,
        Err(_) => return format!("'{}' is not a number", parts[0]),
    };
    let from = match units::find(parts[1]) {
        Some(u) => u,
        None => return format!("unknown unit '{}' (try `list`)", parts[1]),
    };
    let to = match units::find(parts[2]) {
        Some(u) => u,
        None => return format!("unknown unit '{}' (try `list`)", parts[2]),
    };
    match units::convert(value, from, to) {
        Ok(result) => format!("{value} {} = {result:.4} {}", from.1, to.1),
        Err(message) => format!("Error: {message}"),
    }
}

fn print_units() {
    println!("{:<6} {:<12} {}", "Symbol", "Name", "Kind");
    for unit in units::UNITS {
        println!("{:<6} {:<12} {}", unit.0, unit.1, unit.2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn length() {
        let r = units::convert(1.0, units::find("mi").unwrap(), units::find("km").unwrap());
        assert!(close(r.unwrap(), 1.609344));
    }

    #[test]
    fn temperature() {
        let c = units::find("c").unwrap();
        let f = units::find("F").unwrap();
        let k = units::find("k").unwrap();
        assert!(close(units::convert(100.0, c, f).unwrap(), 212.0));
        assert!(close(units::convert(32.0, f, c).unwrap(), 0.0));
        assert!(close(units::convert(0.0, k, c).unwrap(), -273.15));
    }

    #[test]
    fn mixing_kinds_is_an_error() {
        let r = units::convert(1.0, units::find("kg").unwrap(), units::find("m").unwrap());
        assert!(r.is_err());
    }

    #[test]
    fn bad_input_messages() {
        assert_eq!(run_line("ten km mi"), "'ten' is not a number");
        assert!(run_line("1 km parsec").starts_with("unknown unit"));
    }
}
