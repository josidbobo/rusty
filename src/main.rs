use std::env;
use bdk::{Wallet, bitcoin::Network, database::MemoryDatabase, SyncOptions};
use dotenv::from_filename;

fn main() -> anyhow::Result<()>{
    from_filename(".env").ok();

    let descriptor = env::var("WALLET_DESCRIPTOR")?;

    dbg!(&descriptor);

    let wallet = Wallet::new(&descriptor, None, Network::Testnet, MemoryDatabase::default())?;

    dbg!(wallet);
    
    let balance = wallet.get_balance()?;
    let address = wallet.get_address(New)?;

    Ok(())

}
