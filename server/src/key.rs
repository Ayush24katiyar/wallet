

use bip39::Mnemonic;


use solana_derivation_path::DerivationPath;
use solana_sdk::signature::keypair_from_seed_and_derivation_path;
use solana_sdk::signer::Signer;






pub async fn generate_key(seed: &str , index: u32) -> String {

   let phrase = Mnemonic::parse(seed).expect("parsing failed");

   let seed_byte = phrase.to_seed("");
   let abc = format!("m/44'/501'/{}'/0'", index);
   let our_path = DerivationPath::from_absolute_path_str(&abc).expect("invalid derivation path");




   
  let pub_key = keypair_from_seed_and_derivation_path(&seed_byte, Some(our_path)).expect("mismatched happened");
  pub_key.pubkey().to_string()
   

    
}