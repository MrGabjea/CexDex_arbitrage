use crate::live_feed::LiveEvent;
use ethers::prelude::*;
use ethers::types::{Filter, H160, H256, I256, U256};
use futures_util::StreamExt;
use std::str::FromStr;
use tokio::sync::mpsc::Sender;

/// Démarre le flux live des événements Uniswap
pub async fn uniswap_feed(
    wss_address: &str,
    pool_address: &str,
    swap_topic: &str,
    sender: Sender<LiveEvent>,
) -> anyhow::Result<()> {
    // Connexion WebSocket
    let ws = Ws::connect(wss_address).await?;
    let provider = Provider::new(ws);

    println!("Connected to WebSocket");

    // Construction du filtre (address + topic0)
    let filter = Filter::new()
        .address(pool_address.parse::<H160>()?)
        .topic0(H256::from_str(swap_topic)?);

    // Subscription aux logs
    let mut stream = provider.subscribe_logs(&filter).await?;

    println!("Subscribed to Swap events");

    // Boucle asynchrone sur les événements
    while let Some(log) = stream.next().await {
        // println!("Received log");

        let data = log.data.as_ref();
        let block_number = match log.block_number {
            Some(b) => b.as_u64(),
            None => 0,
        };
        let log_index = match log.log_index {
            Some(i) => i.as_u64(),
            None => 0,
        };

        // On vérifie qu'on a au moins 4 slots de 32 bytes
        if data.len() >= 128 {
            let amount0 = parse_int256(&data[0..32]);
            let amount1 = parse_int256(&data[32..64]);
            let sqrt_price_x96 = U256::from_big_endian(&data[64..96]);
            let liquidity = U256::from_big_endian(&data[96..128]);

            let event = LiveEvent::Uniswap {
                block_number,
                log_index,
                amount0,
                amount1,
                sqrt_price_x96,
                liquidity,
            };

            if sender.send(event).await.is_err() {
                return Ok(());
            }
        }
    }

    Ok(())
}

// Helper pour décoder un int256 (complément à deux)
fn parse_int256(bytes: &[u8]) -> I256 {
    I256::from_raw(U256::from_big_endian(bytes))
}
