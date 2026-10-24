// Accounts and the operations on them. Every operation either succeeds
// completely or changes nothing and returns a BankError.

use crate::error::BankError;
use std::collections::BTreeMap;

pub const DAILY_WITHDRAW_LIMIT: u64 = 50_000_00; // Rs 50,000 in paise

#[derive(Debug, Clone, PartialEq)]
pub enum Transaction {
    Deposit(u64),
    Withdrawal(u64),
    TransferIn { from: u32, amount: u64 },
    TransferOut { to: u32, amount: u64 },
}

#[derive(Debug)]
pub struct Account {
    pub id: u32,
    pub owner: String,
    pub balance: u64,
    pub frozen: bool,
    pub withdrawn_today: u64,
    pub history: Vec<Transaction>,
}

#[derive(Default)]
pub struct Bank {
    accounts: BTreeMap<u32, Account>,
    next_id: u32,
}

impl Bank {
    pub fn new() -> Bank {
        Bank { accounts: BTreeMap::new(), next_id: 1001 }
    }

    pub fn open(&mut self, owner: &str, initial: u64) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        let mut account = Account {
            id,
            owner: owner.to_string(),
            balance: 0,
            frozen: false,
            withdrawn_today: 0,
            history: Vec::new(),
        };
        if initial > 0 {
            account.balance = initial;
            account.history.push(Transaction::Deposit(initial));
        }
        self.accounts.insert(id, account);
        id
    }

    pub fn account(&self, id: u32) -> Result<&Account, BankError> {
        self.accounts.get(&id).ok_or(BankError::AccountNotFound(id))
    }

    fn account_mut(&mut self, id: u32) -> Result<&mut Account, BankError> {
        let account = self.accounts.get_mut(&id).ok_or(BankError::AccountNotFound(id))?;
        if account.frozen {
            return Err(BankError::AccountFrozen(id));
        }
        Ok(account)
    }

    pub fn accounts(&self) -> impl Iterator<Item = &Account> {
        self.accounts.values()
    }

    pub fn deposit(&mut self, id: u32, amount: u64) -> Result<u64, BankError> {
        let account = self.account_mut(id)?;
        account.balance += amount;
        account.history.push(Transaction::Deposit(amount));
        Ok(account.balance)
    }

    pub fn withdraw(&mut self, id: u32, amount: u64) -> Result<u64, BankError> {
        let account = self.account_mut(id)?;
        check_can_withdraw(account, amount)?;
        account.balance -= amount;
        account.withdrawn_today += amount;
        account.history.push(Transaction::Withdrawal(amount));
        Ok(account.balance)
    }

    /// Moves money between accounts. All checks happen BEFORE any change,
    /// so a failed transfer never leaves money half-moved.
    pub fn transfer(&mut self, from: u32, to: u32, amount: u64) -> Result<(), BankError> {
        if from == to {
            return Err(BankError::SameAccount);
        }
        self.account_mut(to)?; // receiver must exist and not be frozen
        let sender = self.account_mut(from)?;
        check_can_withdraw(sender, amount)?;

        sender.balance -= amount;
        sender.withdrawn_today += amount;
        sender.history.push(Transaction::TransferOut { to, amount });
        let receiver = self.account_mut(to)?;
        receiver.balance += amount;
        receiver.history.push(Transaction::TransferIn { from, amount });
        Ok(())
    }

    pub fn set_frozen(&mut self, id: u32, frozen: bool) -> Result<(), BankError> {
        let account = self.accounts.get_mut(&id).ok_or(BankError::AccountNotFound(id))?;
        account.frozen = frozen;
        Ok(())
    }

    /// A new day: daily limits reset.
    pub fn new_day(&mut self) {
        for account in self.accounts.values_mut() {
            account.withdrawn_today = 0;
        }
    }
}

fn check_can_withdraw(account: &Account, amount: u64) -> Result<(), BankError> {
    if amount > account.balance {
        return Err(BankError::InsufficientFunds { needed: amount, available: account.balance });
    }
    if account.withdrawn_today + amount > DAILY_WITHDRAW_LIMIT {
        return Err(BankError::DailyLimitExceeded { limit: DAILY_WITHDRAW_LIMIT });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::parse_amount;

    #[test]
    fn deposit_and_withdraw() {
        let mut bank = Bank::new();
        let a = bank.open("Asha", 1000_00);
        assert_eq!(bank.deposit(a, 500_00), Ok(1500_00));
        assert_eq!(bank.withdraw(a, 200_00), Ok(1300_00));
        assert_eq!(
            bank.withdraw(a, 5000_00),
            Err(BankError::InsufficientFunds { needed: 5000_00, available: 1300_00 })
        );
        assert_eq!(bank.account(a).unwrap().history.len(), 3);
    }

    #[test]
    fn failed_transfer_changes_nothing() {
        let mut bank = Bank::new();
        let a = bank.open("Asha", 100_00);
        let b = bank.open("Dev", 0);
        assert!(bank.transfer(a, b, 500_00).is_err());
        assert_eq!(bank.account(a).unwrap().balance, 100_00);
        assert_eq!(bank.account(b).unwrap().balance, 0);
        assert_eq!(bank.transfer(a, 9999, 1_00), Err(BankError::AccountNotFound(9999)));
        assert_eq!(bank.account(a).unwrap().balance, 100_00);
        assert_eq!(bank.transfer(a, a, 1_00), Err(BankError::SameAccount));
    }

    #[test]
    fn transfer_moves_money() {
        let mut bank = Bank::new();
        let a = bank.open("Asha", 100_00);
        let b = bank.open("Dev", 0);
        bank.transfer(a, b, 40_00).unwrap();
        assert_eq!(bank.account(a).unwrap().balance, 60_00);
        assert_eq!(bank.account(b).unwrap().balance, 40_00);
    }

    #[test]
    fn daily_limit_and_frozen() {
        let mut bank = Bank::new();
        let a = bank.open("Rich", 100_000_00);
        bank.withdraw(a, 30_000_00).unwrap();
        assert!(matches!(bank.withdraw(a, 30_000_00), Err(BankError::DailyLimitExceeded { .. })));
        bank.new_day();
        assert!(bank.withdraw(a, 30_000_00).is_ok());
        bank.set_frozen(a, true).unwrap();
        assert_eq!(bank.deposit(a, 1_00), Err(BankError::AccountFrozen(a)));
    }

    #[test]
    fn amounts() {
        assert_eq!(parse_amount("120"), Ok(120_00));
        assert_eq!(parse_amount("120.5"), Ok(120_50));
        assert_eq!(parse_amount("0.05"), Ok(5));
        assert!(parse_amount("12.345").is_err());
        assert!(parse_amount("-5").is_err());
        assert!(parse_amount("abc").is_err());
        assert!(parse_amount("0").is_err());
    }
}
