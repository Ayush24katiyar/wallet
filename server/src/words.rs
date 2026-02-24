use bip39::Mnemonic;
use rand::rngs::OsRng;
use rand::RngCore;
use std::sync::Mutex;

static STORED_SEED: Mutex<Option<String>> = Mutex::new(None);

pub async fn generating_seed() -> String {
    let mut entropy = [0u8;16];
    OsRng.fill_bytes(&mut entropy);

    let mnemonic = Mnemonic::from_entropy(&entropy).expect("failed to generate the words");
    let seed = mnemonic.to_string();

    let mut lock = STORED_SEED.lock().expect("failed to lock the seed");
    *lock = Some(seed.clone());
    seed

}


pub fn get_seed() -> Option<String> {
    let lock = STORED_SEED.lock().expect("seed is not accessed ");
    lock.clone()

}