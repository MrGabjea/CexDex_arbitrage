mod live_feed;
mod market;
mod config;
use live_feed::LiveEvent;
use market::state::MarketState;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // Create Market State
    let mut market_state: MarketState = Default::default();

    // Initialize & Run LiveFeed
    let (tx, mut rx) = mpsc::channel::<LiveEvent>(1024);
    tokio::spawn(live_feed::livefeed::start_livefeed(tx));

    // Central Loop : Triggered when Event received
    while let Some(event) = rx.recv().await {
        market_state.update(&event);
        
        match event {
            LiveEvent::Uniswap { .. } => {
                market_state.is_imbalanced();
            }
            LiveEvent::Hyperliquid { timestamp, levels } => {
                // println!("[HYPER] {} , {:?}",timestamp,levels );
            }
        }
    }
    Ok(())
}