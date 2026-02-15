use super::Swap;
use super::sqrt_price_x96_to_price;
use crate::live_feed::LiveEvent;
use ethers::types::U256;
use serde_json::Value;
use tokio::sync::mpsc;
use crate::config::SPREAD_THRESHOLD;

#[derive(Debug, Default)]
pub struct MarketState {
    pub best_bid: Option<f64>,
    pub best_ask: Option<f64>,
    pub last_price_pool: Option<f64>,
    pub last_swap: Option<Swap>,
}

impl MarketState {
    pub fn update(&mut self, event: &LiveEvent) -> bool {
        let mut change = false;
        match event {
            LiveEvent::Uniswap {
                block_number,
                log_index,
                amount0,
                amount1,
                sqrt_price_x96,
                liquidity,
            } => {
                if let Some(swap) = &self.last_swap {
                    if (swap.block_number < *block_number)
                        || (swap.block_number == *block_number && swap.log_index < *log_index)
                    {
                        self.last_swap = Some(Swap {
                            block_number: *block_number,
                            log_index: *log_index,
                            amount0: *amount0,
                            amount1: *amount1,
                            sqrt_price_x96: *sqrt_price_x96,
                            liquidity: *liquidity,
                        });
                        change = true;
                    }
                } else {
                    self.last_swap = Some(Swap {
                        block_number: *block_number,
                        log_index: *log_index,
                        amount0: *amount0,
                        amount1: *amount1,
                        sqrt_price_x96: *sqrt_price_x96,
                        liquidity: *liquidity,
                    });
                    change = true;
                }
                self.last_price_pool = Some(sqrt_price_x96_to_price(sqrt_price_x96));
            }

            LiveEvent::Hyperliquid { timestamp, levels } => {
                let (new_bid, new_ask) = get_bid_ask(levels);
                if let Some(bid) = &self.best_bid {
                    if *bid != new_bid || self.best_ask.unwrap() != new_ask {
                        self.best_bid = Some(new_bid);
                        self.best_ask = Some(new_ask);
                        change = true;
                        // println!("{:#?} {:#?}", &self.best_ask,&self.best_bid)
                    }
                } else {
                    self.best_bid = Some(new_bid);
                    self.best_ask = Some(new_ask);
                    change = true;
                    // println!("{:#?} {:#?}", &self.best_ask,&self.best_bid)
                }
            }
        }
        return change;
    }

    pub fn imbalance(&self) -> (f64, bool) {
        if let Some(price_bc) = &self.last_price_pool
         {
            let spread_bid: f64 = match &self.best_bid {
                Some(bid) => bid - price_bc,
                _ => 0.0,
            };
            let spread_ask = match &self.best_ask {
                Some(ask) => price_bc - ask,
                _ => 0.0,
            };
            let side: bool;
            let spread = 10000.0*(if spread_ask > spread_bid {
                side = true;
                spread_ask
            } else if spread_bid > 0.0 {
                side = false;
                spread_bid
            } else {
                side = false;
                0.0
            })/ *price_bc;
            return (spread,side);
        }
        else{
        return (0.0, true);
        }
    }
}

fn get_bid_ask(levels: &Vec<Value>) -> (f64, f64) {
    let bid: f64 = (&levels[0][0]["px"].as_str().unwrap()).parse().unwrap();
    let ask: f64 = (&levels[1][0]["px"].as_str().unwrap()).parse().unwrap();
    return (bid, ask);
}
