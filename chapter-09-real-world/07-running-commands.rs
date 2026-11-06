// Chapter 9, Lesson 7: Running other programs with std::process::Command
// Run: cargo run --example ch09-07

use std::process::Command;

fn main() -> std::io::Result<()> {
    // Command is like asking a colleague to do a job and report back:
    // you name the program, pass arguments, and read what it said.
    let output = Command::new("rustc").arg("--version").output()?;
    println!("status ok: {}", output.status.success());
    println!("rustc says: {}", String::from_utf8_lossy(&output.stdout).trim());

    // Arguments are passed one by one, so spaces need no quoting.
    let echo = Command::new("echo").args(["hello", "from a child process"]).output()?;
    print!("echo says: {}", String::from_utf8_lossy(&echo.stdout));

    // Environment variables can be set just for the child.
    let env = Command::new("sh").arg("-c").arg("echo shop=$SHOP").env("SHOP", "Kirana").output()?;
    print!("sh says: {}", String::from_utf8_lossy(&env.stdout));

    // A failing command still returns Ok; check the status code.
    let fail = Command::new("sh").arg("-c").arg("exit 3").status()?;
    println!("exit code: {:?}", fail.code());

    // A missing program is an Err.
    match Command::new("no-such-program-199").output() {
        Ok(_) => println!("unexpected"),
        Err(e) => println!("could not start: {}", e.kind()),
    }
    Ok(())
}
