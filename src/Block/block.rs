use super::transaction::{self, Transaction};
use crate::wallet::Wallet;
use openssl::rsa::{Padding, Rsa};
use sha2::{Digest, Sha512};
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct Block {
    pub miner: String,
    pub reward: u8,
    pub number: u64,
    pub transactions: Vec<Transaction>,
    pub block_hash: String,
    pub nonce: u64,
    pub difficulty: u8,
    pub last_block_hash: Option<String>,
}

/*
 Reward = 50 - (Number Of blocks / 20000) as u8
*/

#[derive(Debug)]
pub enum BlockError {
    InvalidHash,
    InvalidTransaction,
}

impl Error for BlockError {}

impl fmt::Display for BlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}

fn to_hex_string(bytes: Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    strs.join("").to_ascii_lowercase().to_string()
}

fn gen_block_hash(
    nonce: &u64,
    diff: &u8,
    transactions: &Vec<Transaction>,
    miner: &Wallet,
) -> String {
    let bytes = format!(
        "{:?}{}{}{}",
        transactions,
        &nonce,
        &diff,
        miner.address.to_string()
    );

    let mut hasher = Sha512::new();
    hasher.update(bytes.as_bytes());
    to_hex_string(hasher.finalize().to_vec())
}

impl Block {
    pub fn new(
        miner: Wallet,
        number: u64,
        transactions: [Transaction; 1024],
        nonce: u64,
        hash: String,
        difficulty: u8,
        last_block_hash: Option<String>,
    ) -> Result<Block, BlockError> {
        let rep = "0".repeat((difficulty as usize).clone()).to_string();
        if hash.starts_with(&rep) {
            return Err(BlockError::InvalidHash);
        }

        for t in &transactions {
            let payer_pk = match Rsa::public_key_from_pem(t.payer.public_key.as_bytes()) {
                Ok(res) => res,
                Err(e) => return Err(BlockError::InvalidTransaction),
            };

            let mut buf: Vec<u8> = vec![];
            if let Err(e) = payer_pk.public_decrypt(&t.sign, &mut buf, Padding::PKCS1) {
                return Err(BlockError::InvalidTransaction);
            }

            if buf
                != (format!(
                    "{}{}{}{}",
                    &t.payer.address,
                    &t.receiver.address,
                    &t.amount,
                    &t.transaction_id.to_string()
                )
                .as_bytes())
            {
                return Err(BlockError::InvalidTransaction);
            }
        }

        Ok(Block {
            miner: miner.address.to_string(),
            number: number.clone(),
            transactions: transactions.to_vec(),
            nonce: nonce,
            block_hash: hash,
            last_block_hash: last_block_hash,
            reward: (50 - (number / 20000) as u8),
            difficulty: difficulty,
        })
    }
    pub fn mine_block(
        miner: &mut Wallet,
        mempool: &Vec<Transaction>,
        last_block: Option<Block>,
        diff: u8,
    ) -> Block {
        let rep = "0".repeat(diff as usize);
        let mut nonce: u64 = 0;
        let transactions = mempool.get(0..1024).unwrap_or(&mempool).clone().to_owned();
        let mut hash = gen_block_hash(&nonce, &diff, &transactions, miner);
        loop {
            if hash.starts_with(&rep) {
                break;
            }
            nonce += 1;
            hash = gen_block_hash(&nonce, &diff, &transactions, miner);
        }
        let (number, last_block_hash) = match last_block {
            Some(d) => (d.number, Some(d.block_hash)),
            None => (0 as u64, None),
        };
        miner.amount += 50.0;

        Block {
            block_hash: hash,
            difficulty: diff,
            miner: miner.address.to_string(),
            nonce: nonce,
            number: number,
            last_block_hash: last_block_hash,
            reward: 50,
            transactions: transactions,
        }
    }
}
