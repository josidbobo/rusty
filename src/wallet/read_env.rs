use dotenv::{from_filename, dotenv};
use std::env;


pub fn env_settings() -> anyhow::Result<String>{
    from_filename(".env").ok();
    dotenv().ok();

    let descriptor = env::var("WALLET_DESCRIPTOR")?;

    dbg!(&descriptor);

    Ok(descriptor)
}