// Project 5: Bank account system
//
//   cargo run -p bank-system
// Type `help` for commands.

mod bank;
mod error;

use bank::{Bank, Transaction};
use error::{parse_amount, rupees};
use std::error::Error;
use std::io::{self, BufRead, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let mut bank = Bank::new();
    println!("Rust Bank. Type `help` for commands.");
    let stdin = io::stdin();
    loop {
        print!("bank> ");
        io::stdout().flush()?; // an I/O error here ends main with Err
        let mut line = String::new();
        if stdin.lock().read_line(&mut line)? == 0 {
            break;
        }
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.first() == Some(&"quit") {
            break;
        }
        // One place handles every error: print it and keep going.
        match run(&mut bank, &words) {
            Ok(message) => println!("{message}"),
            Err(e) => println!("Error: {e}"),
        }
    }
    println!("Goodbye.");
    Ok(())
}

/// Runs one command. Any error, from parsing or from the bank, is
/// returned with `?` and reported by the caller.
fn run(bank: &mut Bank, words: &[&str]) -> Result<String, Box<dyn Error>> {
    match words {
        [] => Ok(String::new()),
        ["help"] => Ok(HELP.trim_end().to_string()),
        ["open", name] => open(bank, name, "0"),
        ["open", name, amount] => open(bank, name, amount),
        ["deposit", id, amount] => {
            let balance = bank.deposit(id.parse()?, parse_amount(amount)?)?;
            Ok(format!("Deposited. Balance {}", rupees(balance)))
        }
        ["withdraw", id, amount] => {
            let balance = bank.withdraw(id.parse()?, parse_amount(amount)?)?;
            Ok(format!("Please take your cash. Balance {}", rupees(balance)))
        }
        ["transfer", from, to, amount] => {
            bank.transfer(from.parse()?, to.parse()?, parse_amount(amount)?)?;
            Ok("Transfer complete.".to_string())
        }
        ["statement", id] => statement(bank, id.parse()?),
        ["freeze", id] => {
            bank.set_frozen(id.parse()?, true)?;
            Ok(format!("Account {id} frozen."))
        }
        ["unfreeze", id] => {
            bank.set_frozen(id.parse()?, false)?;
            Ok(format!("Account {id} unfrozen."))
        }
        ["accounts"] => Ok(bank
            .accounts()
            .map(|a| format!("{}  {:<10} {:>14}{}", a.id, a.owner, rupees(a.balance), if a.frozen { "  (frozen)" } else { "" }))
            .collect::<Vec<_>>()
            .join("\n")),
        ["newday"] => {
            bank.new_day();
            Ok("A new day begins. Daily limits reset.".to_string())
        }
        _ => Err(format!("unknown command '{}'. Type `help`.", words.join(" ")).into()),
    }
}

fn open(bank: &mut Bank, name: &str, amount: &str) -> Result<String, Box<dyn Error>> {
    let initial = if amount == "0" { 0 } else { parse_amount(amount)? };
    let id = bank.open(name, initial);
    Ok(format!("Opened account {id} for {name} with {}", rupees(initial)))
}

fn statement(bank: &Bank, id: u32) -> Result<String, Box<dyn Error>> {
    let account = bank.account(id)?;
    let mut lines = vec![format!("Statement for {} ({})", account.owner, account.id)];
    for t in &account.history {
        lines.push(match t {
            Transaction::Deposit(a) => format!("  + {:>14}  deposit", rupees(*a)),
            Transaction::Withdrawal(a) => format!("  - {:>14}  withdrawal", rupees(*a)),
            Transaction::TransferIn { from, amount } => format!("  + {:>14}  from {from}", rupees(*amount)),
            Transaction::TransferOut { to, amount } => format!("  - {:>14}  to {to}", rupees(*amount)),
        });
    }
    lines.push(format!("  Balance: {}", rupees(account.balance)));
    Ok(lines.join("\n"))
}

const HELP: &str = "
  open <name> [amount]          open an account
  deposit <id> <amount>         e.g. deposit 1001 250.50
  withdraw <id> <amount>
  transfer <from> <to> <amount>
  statement <id>
  accounts                      list all accounts
  freeze <id> / unfreeze <id>
  newday                        reset daily withdrawal limits
  quit
";
