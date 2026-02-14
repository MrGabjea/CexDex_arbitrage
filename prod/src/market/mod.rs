use ethers::types::{I256, U256};
use crate::config::{TOKEN0_DECIMAL,TOKEN1_DECIMAL};

pub mod state;

#[derive(Debug, Clone)]
pub struct Swap {
    pub block_number: u64,
    pub log_index: u64,
    pub amount0: I256,
    pub amount1: I256,
    pub sqrt_price_x96: U256,
    pub liquidity: U256,
}

pub fn sqrt_price_x96_to_price(sqrt_price_x96: &U256) -> f64 {
    let sqrt_price_f64 = sqrt_price_x96.as_u128() as f64; 
    let q96 = 2_f64.powi(96);
    let price = (sqrt_price_f64 / q96).powi(2);
    price * 10_f64.powi(TOKEN0_DECIMAL-TOKEN1_DECIMAL)
}