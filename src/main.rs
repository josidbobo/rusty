use std::env;
use bdk::{Wallet, bitcoin::Network, database::MemoryDatabase};
use dotenv::from_filename;

fn main() -> anyhow::Result<()>{
    from_filename(".env").ok();

    let descriptor = env::var("WALLET_DESCRIPTOR")?;

    dbg!(&descriptor);

    let wallet = Wallet::new(&descriptor, None, Network::Testnet, MemoryDatabase::default())?;

    dbg!(wallet);
    Ok(())

}
