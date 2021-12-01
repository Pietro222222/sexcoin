use crate::wallet::Wallet;
use std::{error::Error, fmt};

pub struct Transaction {
    pub receiver: Wallet,
    pub payer: Wallet,
    pub amount: f64,
}

#[derive(Debug)]
pub enum TransactionError {
    NotEnoughMoney,
}

impl Error for TransactionError {}

impl fmt::Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}

impl Transaction {
    pub fn new(p: Wallet, r: Wallet, a: f64) -> Result<Self, TransactionError> {
        if p.amount > a {
            return Err(TransactionError::NotEnoughMoney);
        }

        Ok(Transaction {
            amount: a,
            payer: p,
            receiver: r,
        })
    }
}
