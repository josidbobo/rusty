use std::env;
use dotenv::from_filename;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    from_filename(".env").ok();

    let descriptor = env::var("WALLET_DESCRIPTOR")?;

    dbg!(descriptor);

    Ok(())

}
