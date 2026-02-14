use super::{LiveEvent, hyperliquid, uniswap};
use tokio::sync::mpsc::{self, Sender};

pub async fn start_livefeed(sender: Sender<LiveEvent>) -> anyhow::Result<()> {
    const WSS_BLOCKCHAIN: &str = "wss://arbitrum.drpc.org";
    const POOL_ADDRESS: &str = "0xC6962004f452bE9203591991D15f6b388e09E8D0";
    const SWAP_TOPIC: &str = "0xc42079f94a6350d7e6235f29174924f928cc2ac818eb64fed8004e115fbcca67";

    const WSS_URL: &str = "wss://api.hyperliquid.xyz/ws";
    const COIN: &str = "ETH";

    let (tx, mut rx) = mpsc::channel::<LiveEvent>(1024);

    // Spawn des feeds
    tokio::spawn(uniswap::uniswap_feed(
        WSS_BLOCKCHAIN,
        POOL_ADDRESS,
        SWAP_TOPIC,
        sender.clone(),
    ));

    tokio::spawn(hyperliquid::hyperliquid_feed(WSS_URL, COIN, sender.clone()));
    Ok(())
}
