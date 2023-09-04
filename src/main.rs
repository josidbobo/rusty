mod wallet;

fn main() -> anyhow::Result<()>{
    // Call the function for creating wallet
   wallet::impl_wallet::wallet_implementation()
}
