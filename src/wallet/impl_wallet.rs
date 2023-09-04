use bdk::{Wallet, bitcoin::Network, database::MemoryDatabase, SyncOptions, wallet::AddressIndex::New, blockchain::ElectrumBlockchain, electrum_client::Client};
use super::read_env::env_settings;

pub fn wallet_implementation() -> anyhow::Result<()>{
    let descriptor = env_settings()?;
    let wallet = Wallet::new(&descriptor, None, Network::Testnet, MemoryDatabase::default())?;
    let blockchain = ElectrumBlockchain::from(Client::new("ssl://electrum.blockstream.info:60002")?);
    
    wallet.sync(&blockchain, SyncOptions::default())?;

    let balance = wallet.get_balance()?;
    let address = wallet.get_address(New)?;

    Ok(())

}
