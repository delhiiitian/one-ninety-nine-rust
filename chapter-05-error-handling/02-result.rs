// Chapter 5, Lesson 2: Result for errors you expect
// Run: cargo run --example ch05-02

// Result<T, E> is an enum: Ok(T) on success, Err(E) on failure.
fn withdraw(balance: u32, amount: u32) -> Result<u32, String> {
    if amount == 0 {
        Err("amount must be more than zero".to_string())
    } else if amount > balance {
        Err(format!("insufficient funds: balance {balance}, asked {amount}"))
    } else {
        Ok(balance - amount)
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err("cannot divide by zero".to_string());
    }
    Ok(a / b)
}

fn main() {
    let balance = 1000;
    for amount in [300, 0, 5000] {
        match withdraw(balance, amount) {
            Ok(left) => println!("Withdrew {amount}, {left} left"),
            Err(message) => println!("Withdrawal of {amount} failed: {message}"),
        }
    }

    println!("{:?}", divide(10.0, 4.0)); // Ok(2.5)
    println!("{:?}", divide(1.0, 0.0)); // Err("cannot divide by zero")

    // The standard library returns Result for anything that can fail.
    let parsed: Result<i32, _> = "123".parse::<i32>();
    let failed: Result<i32, _> = "12a".parse::<i32>();
    println!("{parsed:?}");
    println!("{failed:?}");

    // Quick checks
    println!("is_ok: {}, is_err: {}", parsed.is_ok(), failed.is_err());

    // A Result must be used. Ignoring one gives a warning:
    // withdraw(10, 20);   // warning: unused `Result` that must be used
}
