use bip39::Mnemonic;
use solana_sdk::signature:: keypair_from_seed;
use solana_sdk::signer::Signer;





pub async fn generate_key(seed: &str) -> String {
   let phrase = Mnemonic::parse(seed).expect("parsing failed");
   let seed_byte = phrase.to_seed("");
   let slice = &seed_byte[0..32];
   let pub_key = keypair_from_seed(slice).expect("failed to derive seed ");
   pub_key.pubkey().to_string()
   

    
}