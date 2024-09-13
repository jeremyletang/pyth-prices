use alloy::primitives::fixed_bytes;
use eyre::Result;
use pyth_prices::get_price;

#[tokio::main]
async fn main() -> Result<()> {
    let price = get_price(
        "https://rpc.gnosischain.com",
        fixed_bytes!("ff61491a931112ddf1bd8147cd1b641375f79f5825126d665480874634fd0ace"),
    )
    .await?;
    println!("ETH price is {price}");

    return Ok(());
}
