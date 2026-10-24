# Project 5: Bank account system

Open accounts, deposit, withdraw and transfer money with real rules:
no overdrafts, a daily withdrawal limit, frozen accounts, and transfers that
either fully happen or not at all. Every failure is a typed `BankError`.
Built in Chapter 5 of *Rust in 199 Pages*.

```bash
cargo run -p bank-system
bank> open Asha 1000
bank> open Dev
bank> transfer 1001 1002 250.50
bank> withdraw 1002 999
Error: insufficient funds: needed Rs 999.00, available Rs 250.50
bank> statement 1001
cargo test -p bank-system
```

## Layout

```text
05-bank-system/src/
├── main.rs    command loop; one place that reports every error
├── bank.rs    Bank, Account, Transaction and the rules, plus tests
└── error.rs   BankError (Display + Error), money parsing and formatting
```

## Ideas to extend

- Interest: a `month` command that adds 0.5% to every balance.
- Save accounts to a file and load them at start (Chapter 6).
- A PIN per account, with the account frozen after 3 wrong tries.
