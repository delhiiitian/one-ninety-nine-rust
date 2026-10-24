// Every way a banking operation can fail, in one enum.

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum BankError {
    AccountNotFound(u32),
    InsufficientFunds { needed: u64, available: u64 },
    InvalidAmount(String),
    DailyLimitExceeded { limit: u64 },
    SameAccount,
    AccountFrozen(u32),
}

impl fmt::Display for BankError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BankError::AccountNotFound(id) => write!(f, "account {id} does not exist"),
            BankError::InsufficientFunds { needed, available } => write!(
                f,
                "insufficient funds: needed {}, available {}",
                rupees(*needed),
                rupees(*available)
            ),
            BankError::InvalidAmount(text) => write!(f, "'{text}' is not a valid amount"),
            BankError::DailyLimitExceeded { limit } => {
                write!(f, "daily withdrawal limit of {} exceeded", rupees(*limit))
            }
            BankError::SameAccount => write!(f, "cannot transfer to the same account"),
            BankError::AccountFrozen(id) => write!(f, "account {id} is frozen"),
        }
    }
}

impl std::error::Error for BankError {}

/// Money is stored in paise (u64) to avoid floating-point errors.
pub fn rupees(paise: u64) -> String {
    format!("Rs {}.{:02}", paise / 100, paise % 100)
}

/// Parses "120", "120.5" or "120.50" into paise.
pub fn parse_amount(text: &str) -> Result<u64, BankError> {
    let bad = || BankError::InvalidAmount(text.to_string());
    let (whole, frac) = text.split_once('.').unwrap_or((text, "0"));
    if whole.is_empty() || frac.len() > 2 || !frac.chars().all(|c| c.is_ascii_digit()) {
        return Err(bad());
    }
    let whole: u64 = whole.parse().map_err(|_| bad())?;
    let frac: u64 = format!("{frac:0<2}").parse().map_err(|_| bad())?;
    let paise = whole.checked_mul(100).and_then(|w| w.checked_add(frac)).ok_or_else(bad)?;
    if paise == 0 {
        return Err(bad());
    }
    Ok(paise)
}
