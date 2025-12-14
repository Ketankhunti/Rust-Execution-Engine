use crate::engine::order_book::{Price, Quantity};
use crate::domain::order::OrderId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trade {
    pub price: Price,
    pub quantity: Quantity,
    pub buy_order_id: OrderId,
    pub sell_order_id: OrderId,
}
