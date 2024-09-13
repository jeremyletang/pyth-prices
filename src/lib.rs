use alloy::primitives::{address, Address, FixedBytes, I256};
use alloy::providers::ProviderBuilder;
use alloy::sol;
use eyre::Result;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    PythRebase,
    "abi/PythRebase.json"
);

const CONTRACT_ADDRESS: Address = address!("cD0FC7f4013f48fED536918929f75487E6690E69");

pub async fn get_jpy_usdt_price(rpc_url: &str) -> Result<I256> {
    let rpc_url = rpc_url.parse()?;

    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().on_http(rpc_url);
    let contract = PythRebase::new(CONTRACT_ADDRESS, provider);

    let price = contract.getJpyUsdtPrice().call().await?._0;

    return Ok(price);
}

pub async fn get_price(rpc_url: &str, id: FixedBytes<32>) -> Result<I256> {
    let rpc_url = rpc_url.parse()?;

    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().on_http(rpc_url);
    let contract = PythRebase::new(CONTRACT_ADDRESS, provider);

    let price = contract.getPrice_1(id).call().await?._0;

    return Ok(price);
}
