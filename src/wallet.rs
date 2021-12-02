use openssl::rsa::{Padding, Rsa};
use openssl::symm::Cipher;
use rand::rngs::OsRng;
use rand::Rng;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Result, Write};
use std::rc::Rc;
use uuid::Uuid;

fn generate_random_word(length: usize) -> String {
    const CHAR_SET: &[u8] = b"abcdefghijklmnopqrstuvwxyz\
        ABCDEFGHIJKLMNOPQRSTUVWXYZ\
        0123456789(~!@#$%^&*)";

    let mut rnd = rand::thread_rng();

    let password: String = (0..length)
        .map(|_| {
            let idx = rnd.gen_range(0..CHAR_SET.len());
            CHAR_SET[idx] as char
        })
        .collect();
    password
}

pub fn generate_keys() -> Result<(String, String, String)> /*Public, private key, PassPhrase*/ {
    let passphrase = generate_random_word(32);

    let rsa = Rsa::generate(1024)?;

    let private_key: Vec<u8> =
        rsa.private_key_to_pem_passphrase(Cipher::aes_128_cbc(), passphrase.as_bytes())?;
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(".private_key")
        .unwrap()
        .write(&private_key);

    OpenOptions::new()
        .create(true)
        .write(true)
        .open(".passphrase")
        .unwrap()
        .write(&passphrase.as_bytes());

    let public_key: Vec<u8> = rsa.public_key_to_pem()?;
    Ok((
        String::from_utf8(public_key).unwrap(),
        String::from_utf8(private_key).unwrap(),
        passphrase,
    ))
}

#[derive(Debug, Clone)]
pub struct Wallet {
    pub amount: f64,
    pub address: Uuid,
    pub public_key: String,
}

impl Wallet {
    pub fn new(wallets: &mut HashMap<String, Rc<RefCell<Wallet>>>) -> Result<String> {
        let keys = generate_keys()?;

        println!("PRIVATE KEY: {}", keys.1);
        let address = Uuid::new_v4();

        let w = Rc::new(RefCell::new(Wallet {
            amount: 0.0,
            address: address.clone(),
            public_key: keys.0.clone(),
        }));
        let rc_wallet = wallets.insert(address.to_string(), Rc::clone(&w));
        Ok(address.to_string())
    }
}
