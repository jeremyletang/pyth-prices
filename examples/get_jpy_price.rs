use eyre::Result;
use pyth_prices::get_jpy_usdt_price;

#[tokio::main]
async fn main() -> Result<()> {
    let price = get_jpy_usdt_price("https://rpc.gnosischain.com").await?;
    println!("JPY price is {price}");

    return Ok(());
}
