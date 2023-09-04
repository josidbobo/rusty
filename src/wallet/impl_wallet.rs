use std::env;
use bdk::{Wallet, bitcoin::Network, database::MemoryDatabase, SyncOptions, wallet::AddressIndex::New};
use dotenv::from_filename;

pub fn wallet_implementation() -> anyhow::Result<()>{
    from_filename(".env").ok();

    let descriptor = env::var("WALLET_DESCRIPTOR")?;

    dbg!(&descriptor);

    let wallet = Wallet::new(&descriptor, None, Network::Testnet, MemoryDatabase::default())?;

    dbg!(&wallet);
    
    let balance = wallet.get_balance()?;
    let address = wallet.get_address(New)?;

    dbg!(address, balance);
    Ok(())

}
