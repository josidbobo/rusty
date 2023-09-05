use std::{net::SocketAddr, path::Path};
use bdk::{Wallet, bitcoin::Network, database::SqliteDatabase, SyncOptions, blockchain::ElectrumBlockchain, electrum_client::Client};
use axum::{Json, routing::get, Router, response::IntoResponse};

use super::{read_env::env_settings, structs_impl::{AddressResponse, AppError}};

#[tokio::main]
pub async fn wallet_implementation() -> anyhow::Result<()> {
    let my_path = Path::new("maazi.db");

    let descriptor = env_settings()?;
    let wallet = Wallet::new(
        &descriptor, None,
        Network::Testnet, SqliteDatabase::new(my_path),
    )?;

    let blockchain = ElectrumBlockchain::from(Client::new("ssl://electrum.blockstream.info:60002")?);

    //let balance = wallet.get_balance()?;
    //let address = wallet.get_address(New)?;

    let app = Router::new().route("/", get(handler));

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
    wallet.sync(&blockchain, SyncOptions::default())?;

    Ok(())

}

async fn handler() -> Result<impl IntoResponse, AppError>{
    let response = AddressResponse{
        address: "test".to_string(),
        index: 0,
    };

    Ok(Json(response))
}



