// Chapter 5, Lesson 8: Converting errors with From so ? just works
// Run: cargo run --example ch05-08

use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug)]
enum ConfigError {
    BadNumber(ParseIntError),
    BadDecimal(ParseFloatError),
    Missing(&'static str),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::BadNumber(e) => write!(f, "expected a whole number ({e})"),
            ConfigError::BadDecimal(e) => write!(f, "expected a decimal number ({e})"),
            ConfigError::Missing(key) => write!(f, "missing setting '{key}'"),
        }
    }
}

impl std::error::Error for ConfigError {}

// With these impls, `?` converts the library error into ConfigError for us.
impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::BadNumber(e)
    }
}

impl From<ParseFloatError> for ConfigError {
    fn from(e: ParseFloatError) -> Self {
        ConfigError::BadDecimal(e)
    }
}

#[derive(Debug)]
struct Config {
    port: u16,
    workers: u32,
    tax_rate: f64,
}

fn get<'a>(lines: &[(&'static str, &'a str)], key: &'static str) -> Result<&'a str, ConfigError> {
    lines.iter().find(|(k, _)| *k == key).map(|(_, v)| *v).ok_or(ConfigError::Missing(key))
}

fn load(lines: &[(&'static str, &str)]) -> Result<Config, ConfigError> {
    let port = get(lines, "port")?.parse()?; // ParseIntError -> ConfigError
    let workers = get(lines, "workers")?.parse()?;
    let tax_rate = get(lines, "tax_rate")?.parse()?; // ParseFloatError -> ConfigError
    Ok(Config { port, workers, tax_rate })
}

fn main() {
    let good = [("port", "8080"), ("workers", "4"), ("tax_rate", "0.18")];
    let bad_port = [("port", "eighty"), ("workers", "4"), ("tax_rate", "0.18")];
    let bad_rate = [("port", "8080"), ("workers", "4"), ("tax_rate", "18%")];
    let missing = [("port", "8080")];

    for (name, cfg) in [("good", &good[..]), ("bad port", &bad_port[..]), ("bad rate", &bad_rate[..]), ("missing", &missing[..])] {
        match load(cfg) {
            Ok(c) => println!("{name:>9}: port {}, {} workers, tax {}", c.port, c.workers, c.tax_rate),
            Err(e) => println!("{name:>9}: error: {e}"),
        }
    }
}
