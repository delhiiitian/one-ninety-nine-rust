// Chapter 5, Lesson 6: Your own error type
// Run: cargo run --example ch05-06

// An enum lists every way an operation can fail. Callers can match on it
// and react differently to each case, which a String cannot offer.
#[derive(Debug, PartialEq)]
enum SignupError {
    UsernameTooShort { min: usize },
    UsernameTaken(String),
    WeakPassword(Vec<&'static str>), // the rules that were broken
    TooYoung { age: u32 },
}

fn sign_up(existing: &[&str], username: &str, password: &str, age: u32) -> Result<String, SignupError> {
    if username.len() < 3 {
        return Err(SignupError::UsernameTooShort { min: 3 });
    }
    if existing.contains(&username) {
        return Err(SignupError::UsernameTaken(username.to_string()));
    }
    let mut problems = Vec::new();
    if password.len() < 8 {
        problems.push("at least 8 characters");
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        problems.push("a digit");
    }
    if !password.chars().any(|c| c.is_uppercase()) {
        problems.push("an uppercase letter");
    }
    if !problems.is_empty() {
        return Err(SignupError::WeakPassword(problems));
    }
    if age < 13 {
        return Err(SignupError::TooYoung { age });
    }
    Ok(format!("Welcome aboard, {username}!"))
}

fn main() {
    let existing = ["asha", "dev"];
    let attempts = [
        ("al", "Secret123", 20),
        ("asha", "Secret123", 20),
        ("zoya", "secret", 20),
        ("kabir", "Secret123", 11),
        ("meera", "Secret123", 25),
    ];
    for (user, pass, age) in attempts {
        match sign_up(&existing, user, pass, age) {
            Ok(msg) => println!("{msg}"),
            Err(SignupError::UsernameTooShort { min }) => println!("{user}: use at least {min} letters"),
            Err(SignupError::UsernameTaken(name)) => println!("{name}: taken, try {name}{age}"),
            Err(SignupError::WeakPassword(rules)) => println!("{user}: password needs {}", rules.join(", ")),
            Err(SignupError::TooYoung { age }) => println!("{user}: must be 13+, you are {age}"),
        }
    }
}
