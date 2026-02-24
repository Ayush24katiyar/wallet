use axum::Router ;
use axum::routing::get;
use axum::serve;
use tokio::net::TcpListener;

use crate::handler::{health , wallet_handler , generate_seed_wallet};


mod handler;
mod words;
mod key;




#[tokio::main]
async fn main () {
    let listener = TcpListener::bind("127.0.0.1:3000").await.expect("Dear Users and binding vow is expired");
    println!("the server is running on 127.0.0.1:3000");

    let app = Router::new()
    .route("/", get(health))
    .route("/seed",get(generate_seed_wallet))
    .route("/wallet",get(wallet_handler));

    serve(listener, app).await.expect("server is failed to connect");
}