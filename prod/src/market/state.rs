use super::Swap;
use crate::live_feed::LiveEvent;
use ethers::types::U256;
use tokio::sync::mpsc;
use serde_json::Value;
use super::sqrt_price_x96_to_price;

#[derive(Debug, Default)]
pub struct MarketState {
    pub best_bid: Option<f64>,
    pub best_ask: Option<f64>,
    pub last_swap: Option<Swap>,
}

impl MarketState {
    pub fn update(&mut self, event: &LiveEvent) {
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
                    }
                }
                else {
                    self.last_swap = Some(Swap {
                            block_number: *block_number,
                            log_index: *log_index,
                            amount0: *amount0,
                            amount1: *amount1,
                            sqrt_price_x96: *sqrt_price_x96,
                            liquidity: *liquidity,
                        });
                }
            }

            LiveEvent::Hyperliquid { 
                timestamp,
                levels,
             } => {
                let (new_bid,new_ask) = get_bid_ask(levels);
                if let Some(bid) = &self.best_bid {
                    if *bid != new_bid || self.best_ask.unwrap() != new_ask {
                        self.best_bid = Some(new_bid);
                        self.best_ask = Some(new_ask);
                        // println!("{:#?} {:#?}", &self.best_ask,&self.best_bid)
                    }
                }
                else {
                    self.best_bid = Some(new_bid);
                    self.best_ask = Some(new_ask);
                    // println!("{:#?} {:#?}", &self.best_ask,&self.best_bid)
                }
             }
        }
    }

    pub fn is_imbalanced(&self) -> (bool,bool) {
        if let Some(swap) = &self.last_swap {
            let price_pool = sqrt_price_x96_to_price(&swap.sqrt_price_x96);
            println!("{:?}",&price_pool)

        }
        return (true,true);
    }
}

fn get_bid_ask(levels: &Vec<Value>) -> (f64,f64) {
    let bid : f64 = (&levels[0][0]["px"].as_str().unwrap()).parse().unwrap();
    let ask: f64 = (&levels[1][0]["px"].as_str().unwrap()).parse().unwrap();
 return (bid,ask)
}
