use ethers::prelude::*;
use ethers::types::{Address, U256};
use std::env;
use std::sync::Arc;
use std::time::Instant;
use std::time::{SystemTime, UNIX_EPOCH};

abigen!(ISwapRouter, "./src/abis/ISwapRouter.json");

pub async fn swap_exact_input() -> anyhow::Result<()> {
    println!("Hello");

    // =============================
    // CONFIG
    // =============================

    let rpc_url = env::var("ARBITRUM_RPC_URL")?;
    let private_key = env::var("PRIVATE_KEY")?;

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let wallet: LocalWallet = private_key.parse::<LocalWallet>()?.with_chain_id(42161u64); // Arbitrum One chain ID

    let block_number = provider.get_block_number().await;
    println!("Dernier block miné : {:?}", block_number);

    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    let t1 = Instant::now();
    // Adresse du router Uniswap V3 sur Arbitrum
    let router_address: Address = "0xE592427A0AEce92De3Edee1F18E0157C05861564".parse()?; // SwapRouter02

    let router = ISwapRouter::new(router_address, client.clone());

    // =============================
    // PARAMÈTRES DU SWAP
    // =============================

    let token_in: Address = "0x912CE59144191C1204E64559FE8253a0e49E6548".parse()?;
    let token_out: Address = "0x82aF49447D8a07e3bd95BD0d56f35241523fBab1".parse()?;

    let fee: u32 = 3000;
    let recipient: Address = "0x6A87aD4C11fc34A2161727BBdD936d04E7A6fc9b".parse()?;

    let deadline = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 600; // 10 minutes

    let amount_in = U256::from_dec_str("100000000000000000")?;
    let amount_out_minimum = U256::from(1); // ⚠️ mettre un vrai slippage !
    let sqrt_price_limit_x96 = U256::from(0); // 0 = pas de limite

    // =============================
    // CALL exactInputSingle
    // =============================

    let params = i_swap_router::ExactInputSingleParams {
        token_in,
        token_out,
        fee,
        recipient: client.address(),
        deadline: U256::from(deadline),
        amount_in,
        amount_out_minimum,
        sqrt_price_limit_x96: U256::zero(),
    };

    let tx = router.exact_input_single(params).value(U256::zero()); // Mettre >0 si tokenIn = WETH/ETH natif

    let pending_tx = tx.send().await?;
    let t2 = Instant::now();
    println!("temps: {:?}", t2 - t1);

    println!("Transaction envoyée: {:?}", pending_tx.tx_hash());

    // let receipt = pending_tx.await?;
    // println!("Receipt: {:?}", receipt);

    Ok(())
}
