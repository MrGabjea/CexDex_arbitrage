mod config;
mod live_feed;
mod market;
mod orders;
use live_feed::LiveEvent;
use market::state::MarketState;
use tokio::sync::mpsc;
// use tokio::time::{sleep,Duration};
use crate::config::SPREAD_END_OF_TRADE;
use crate::config::SPREAD_THRESHOLD;
use dotenv::dotenv;
use std::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    // Create Market State
    let mut market_state: MarketState = Default::default();

    // Initialize & Run LiveFeed
    let (tx, mut rx) = mpsc::channel::<LiveEvent>(1024);
    tokio::spawn(live_feed::livefeed::start_livefeed(tx));

    let mut in_trade = false;
    let mut time = Instant::now();

    let mut tp = 0.0;
    let mut sl = 0.0;
    let mut side_trade = false;
    // tokio::time::sleep(Duration::from_secs(5)).await;
    // let mut time = Instant::now();
    // let transc = orders::swap::swap_exact_input().await;

    // Central Loop : Triggered when Event received
    while let Some(event) = rx.recv().await {
        let change = market_state.update(&event);
        let (spread, side) = market_state.imbalance();
        if spread > SPREAD_THRESHOLD && !(in_trade) {
            in_trade = true;
            time = Instant::now();
            (tp, sl) = match side {
                false => (
                    (*(&market_state.best_ask)).unwrap() - 1.9,
                    (*(&market_state.best_bid)).unwrap() + 1.5,
                ),
                true => (
                    (*(&market_state.best_bid)).unwrap() + 1.9,
                    (*(&market_state.best_ask)).unwrap() - 1.5,
                ),
            };
            println!(" Trade Starting :");
            println!("tp: {:?}, sl: {:?}", &tp, &sl);
            println!(
                "bid: {:?} , ask: {:?}, pool_price: {:?}",
                &market_state.best_bid, &market_state.best_ask, &market_state.last_price_pool
            );
            // println!("Opportunité de trade trouvée !")
        }
        if change && in_trade {
            let t = Instant::now();
            println!(
                "time: {:?}, bid: {:?} , ask: {:?}, pool_price: {:?}",
                t - *(&time),
                &market_state.best_bid,
                &market_state.best_ask,
                &market_state.last_price_pool
            );
        }
        if in_trade {
            let bid = (*(&market_state.best_bid)).unwrap();
            // let ask = (*(&market_state.best_ask)).unwrap();
            if (bid > tp && bid > sl) || (bid < tp && bid < sl) {
                in_trade = false;
                println!("End of Trade \n");
            }
        }

        // if spread<SPREAD_END_OF_TRADE && in_trade{
        //     time = Instant::now();
        //     in_trade = false;
        // }

        // // }
        // // println!("side: {}, spread: {}", side,spread);
        // // println!("best bid {:?} , best ask {:?}, price pool : {:?}", &market_state.best_bid,&market_state.best_ask, &market_state.last_price_pool);
        // match event {
        //     LiveEvent::Uniswap {
        //         block_number,
        //         log_index,
        //         amount0,.. } => {
        //         println!("blocknumber: {}, amount0: {:?}",block_number,amount0);
        //         let t = Instant::now();
        //         println!("temps pour avoir  {:?}", t-time);
        //         time = Instant::now();
        //         return Ok(());

        //         // market_state.is_imbalanced();

        //     }
        //     LiveEvent::Hyperliquid { timestamp, levels } => {
        //         // println!("[HYPER] {} , {:?}",timestamp,levels );
        //     }
        // }
    }
    Ok(())
}
