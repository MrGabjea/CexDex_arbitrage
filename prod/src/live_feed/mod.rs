use ethers::types::{I256, U256};

pub mod livefeed;
mod uniswap;
mod hyperliquid;

#[derive(Debug, Clone)]
enum LiveEvent {
    Uniswap {
        block_number: u64,
        log_index: u64,
        amount0: I256,
        amount1: I256,
        sqrt_price_x96: U256,
        liquidity: U256,
    },
    Hyperliquid {
        coin: String,
        raw_message: String,
    },
}
