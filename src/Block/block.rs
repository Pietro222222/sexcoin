use super::transaction::Transaction;
use crate::wallet::Wallet;

use std::{error::Error, fmt, ptr::null};

pub struct Block {
    pub miner: Wallet,
    pub reward: u8,
    pub number: u64,
    pub transactions: [Transaction; 1024],
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
}

impl Error for BlockError {}

impl fmt::Display for BlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
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

        Ok(Block {
            miner: miner,
            number: number.clone(),
            transactions: transactions,
            nonce: nonce,
            block_hash: hash,
            last_block_hash: last_block_hash,
            reward: (50 - (number / 20000) as u8),
            difficulty: difficulty,
        })
    }
}