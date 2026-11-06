// Chapter 7, Lesson 7: Unit tests with #[test]
// Run the program:  cargo run --example ch07-07
// Run the tests:    cargo test --example ch07-07

/// Returns true if the text reads the same forwards and backwards,
/// ignoring case, spaces and punctuation.
fn is_palindrome(text: &str) -> bool {
    let letters: Vec<char> = text.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_ascii_lowercase()).collect();
    letters.iter().eq(letters.iter().rev())
}

/// Indian-style grouping: 12,34,567
fn format_inr(n: u64) -> String {
    let s = n.to_string();
    if s.len() <= 3 {
        return s;
    }
    let (head, last3) = s.split_at(s.len() - 3);
    let mut groups = Vec::new();
    let mut rest = head;
    while rest.len() > 2 {
        let (a, b) = rest.split_at(rest.len() - 2);
        groups.push(b);
        rest = a;
    }
    groups.push(rest);
    groups.reverse();
    format!("{},{last3}", groups.join(","))
}

fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 { Err("division by zero".into()) } else { Ok(a / b) }
}

fn main() {
    for t in ["Malayalam", "Was it a car or a cat I saw?", "Rust"] {
        println!("{t:?} palindrome? {}", is_palindrome(t));
    }
    println!("Rs {}", format_inr(1234567));
    println!("10 / 0 -> {:?}", safe_divide(10, 0));
    println!("Run `cargo test --example ch07-07` to run the tests below.");
}

// #[cfg(test)] means: only compile this module when running tests.
#[cfg(test)]
mod tests {
    use super::*; // bring the functions above into the test module

    #[test]
    fn palindromes() {
        assert!(is_palindrome("Malayalam"));
        assert!(is_palindrome("Was it a car or a cat I saw?"));
        assert!(!is_palindrome("Rust"));
    }

    #[test]
    fn indian_number_format() {
        assert_eq!(format_inr(999), "999");
        assert_eq!(format_inr(1000), "1,000");
        assert_eq!(format_inr(100000), "1,00,000");
        assert_eq!(format_inr(1234567), "12,34,567");
    }

    #[test]
    fn divide_errors() {
        assert_eq!(safe_divide(10, 2), Ok(5));
        assert!(safe_divide(1, 0).is_err());
    }

    // A test can return Result and use ?
    #[test]
    fn using_question_mark() -> Result<(), String> {
        let v = safe_divide(9, 3)?;
        assert_eq!(v, 3);
        Ok(())
    }

    // This test passes only if the code panics.
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn out_of_bounds_panics() {
        let v = vec![1, 2, 3];
        let i = v.len() + 1;
        let _ = v[i];
    }
}
