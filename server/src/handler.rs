use axum::extract::Path;

use crate::words::{generating_seed , get_seed};
use crate::key::generate_key;



pub async fn health() -> &'static str {
   " Server is running"
   
}
pub async fn generate_seed_wallet () -> String {
   let seed = generating_seed().await;
   seed
}

pub async fn wallet_handler (Path(index): Path<u32>) -> String {
   let seed_option = get_seed();
   match seed_option {
      Some(seed) => {
         let wallet = generate_key(&seed , index).await;
         wallet
      }
      None => {
         "seed not generated yet".to_string()
      }
   }
}