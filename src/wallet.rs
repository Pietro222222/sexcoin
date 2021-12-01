use aes::cipher::{
    generic_array::GenericArray, BlockCipher, BlockDecrypt, BlockEncrypt, NewBlockCipher,
};
use aes::{Aes128, Block, ParBlocks};
use rand::Rng;
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

pub fn generate_keys() -> (Vec<u8>, String, String) /*Aes, Public, private key*/ {
    let public_key = generate_random_word(64);
    let private_key = generate_random_word(16);
    let private_key_bytes = GenericArray::from_slice(private_key.as_bytes());

    /*
     *  Aes = Public key
     *  Public key = Aes but decrypted with the Private Key
     */

    let mut block = Block::default();
    let mut block8 = ParBlocks::default();
    let cipher = Aes128::new(&private_key_bytes);

    let block_copy = block.clone();

    let encrypted = cipher.encrypt_block(&mut block);

    (block.to_vec(), public_key, private_key)
}

pub struct Wallet {
    pub amount: f64,
    pub address: Uuid,
    pub public_key: String, //SHA256 Encrypted text
    pub sign: Vec<u8>,
}

impl Wallet {
    pub fn new(key: String) -> Self {
        let keys = generate_keys();
        println!("PRIVATE KEY: {}", keys.2);
        Wallet {
            amount: 0.0,
            address: Uuid::new_v4(),
            public_key: keys.1,
            sign: keys.0,
        }
    }
}
