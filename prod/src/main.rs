mod live_feed;
mod market;
mod config;
use live_feed::LiveEvent;
use market::state::MarketState;
use tokio::sync::mpsc;
use std::time::Instant;
use crate::config::SPREAD_THRESHOLD;
use crate::config::SPREAD_END_OF_TRADE;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // Create Market State
    let mut market_state: MarketState = Default::default();

    // Initialize & Run LiveFeed
    let (tx, mut rx) = mpsc::channel::<LiveEvent>(1024);
    tokio::spawn(live_feed::livefeed::start_livefeed(tx));

    let mut time = Instant::now(); 
    let mut in_trade = false;

    // Central Loop : Triggered when Event received
    while let Some(event) = rx.recv().await {
        let change = market_state.update(&event);
        let (spread,side) = market_state.imbalance();
        if spread>SPREAD_THRESHOLD && !(in_trade) {
            in_trade = true;
            time = Instant::now();
            println!(" Etat début : \n
            bid: {:?} , ask: {:?}, pool_price: {:?}",
            &market_state.best_bid,
            &market_state.best_ask,
            &market_state.last_price_pool);
            println!("Opportunité de trade trouvée !")
        }
        if change && in_trade{
            println!("bid: {:?} , ask: {:?}, pool_price: {:?}",
            &market_state.best_bid,
            &market_state.best_ask,
            &market_state.last_price_pool);
        }
            
        if spread<SPREAD_END_OF_TRADE && in_trade{
            let t = Instant::now();
            println!("Etat fin: \n 
            bid: {:?} , ask: {:?}, pool_price: {:?}",
            &market_state.best_bid,
            &market_state.best_ask,
            &market_state.last_price_pool);
            println!("Durée de l'opportunité : {:?} \n", t-time);
            time = Instant::now();
            in_trade = false;
        }



        // }
        // println!("side: {}, spread: {}", side,spread);
        // println!("best bid {:?} , best ask {:?}, price pool : {:?}", &market_state.best_bid,&market_state.best_ask, &market_state.last_price_pool);
        // match event {
        //     LiveEvent::Uniswap { .. } => {
        //         let t = Instant::now();
        //         println!("temps entre swap {:?}", t-time);
        //         time = Instant::now();

        //         // market_state.is_imbalanced();

        //     }
        //     LiveEvent::Hyperliquid { timestamp, levels } => {
        //         // println!("[HYPER] {} , {:?}",timestamp,levels );
        //     }
        // }
    }
    Ok(())
}