use crate::engine::order_book::{Price, Quantity};

pub type OrderId = u64;
pub type Sequence = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType{
    Limit,
    Market
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: OrderId,
    pub side: Side,
    pub order_type: OrderType,
    pub price: Option<Price>,  // None for Market orders
    pub quantity: Quantity,
    pub sequence: Sequence, // deterministic ordering
}

