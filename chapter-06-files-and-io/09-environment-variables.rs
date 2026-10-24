// Chapter 6, Lesson 9: Environment variables
// Run: cargo run --example ch06-09
// Try:  APP_PORT=9000 APP_DEBUG=true cargo run --example ch06-09

use std::env;

fn main() {
    // env::var returns Result: Err when the variable is not set.
    match env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
        Ok(home) => println!("Your home folder: {home}"),
        Err(_) => println!("No home folder variable set"),
    }

    // Settings with defaults: the twelve-factor app style.
    let port: u16 = env::var("APP_PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080);
    let debug = env::var("APP_DEBUG").map(|v| v == "true" || v == "1").unwrap_or(false);
    let name = env::var("APP_NAME").unwrap_or_else(|_| "rust199".to_string());
    println!("Starting {name} on port {port} (debug: {debug})");

    // Secrets such as API keys belong in the environment, not in code.
    match env::var("APP_API_KEY") {
        Ok(key) => println!("API key loaded ({} characters)", key.len()),
        Err(_) => println!("APP_API_KEY not set: running in offline mode"),
    }

    // List variables that start with APP_
    let mut app_vars: Vec<(String, String)> = env::vars().filter(|(k, _)| k.starts_with("APP_")).collect();
    app_vars.sort();
    println!("APP_ variables found: {}", app_vars.len());

    // The current directory and the program's own path
    if let Ok(dir) = env::current_dir() {
        println!("Running in {}", dir.display());
    }
}
