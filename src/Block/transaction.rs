use crate::wallet::Wallet;
use openssl::rsa::{Padding, Rsa};
use rand::rngs::OsRng;
use std::{error::Error, fmt};

pub struct Transaction {
    pub receiver: Wallet,
    pub payer: Wallet,
    pub amount: f64,
    pub sign: Vec<u8>,
}

#[derive(Debug)]
pub enum TransactionError {
    NotEnoughMoney,
    SignError,
    InvalidAmount,
}

impl Error for TransactionError {}

impl fmt::Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transaction")
    }
}

impl Transaction {
    pub fn new(
        p: Wallet,
        payer_private_key: String,
        r: Wallet,
        a: f64,
    ) -> Result<Self, TransactionError> {
        if p.amount > a {
            return Err(TransactionError::NotEnoughMoney);
        }

        if p.amount <= 0.0 {
            return Err(TransactionError::InvalidAmount);
        }
        let mut rng = OsRng;

        let rsa = match Rsa::private_key_from_pem(p.address.as_bytes()) {
            Ok(d) => d,
            Err(e) => {
                return Err(TransactionError::SignError);
            }
        };

        let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
        if let Err(e) = rsa.private_encrypt(
            format!("{}{}{}", &p.address, &r.address, &a).as_bytes(),
            &mut buf,
            Padding::PKCS1,
        ) {
            return Err(TransactionError::SignError);
        }

        Ok(Transaction {
            amount: a,
            payer: p,
            receiver: r,
            sign: buf,
        })
    }
}
