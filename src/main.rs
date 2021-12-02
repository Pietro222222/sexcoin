mod Block;
mod wallet;
use rsa::Hash;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wallet::Wallet;
use Block::block;
use Block::transaction::Transaction;

fn main() {
    let mut wallets: HashMap<String, Rc<RefCell<Wallet>>> = HashMap::new();
    let mut mempool: Vec<Transaction> = vec![];
    let mywallet_address = wallet::Wallet::new(&mut wallets).unwrap();
    let mywallet = wallets.get(&mywallet_address).unwrap();
    println!("{:?}", wallets);
    println!("{:?}", mywallet)
}
