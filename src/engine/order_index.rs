use std::collections::HashMap;

use crate::{domain::Side, engine::order_book::Price};


pub type OrderId = u64;

#[derive(Debug, Clone, Copy)]
pub struct OrderLocation  {
    pub side: Side,
    pub price: Price,
}

#[derive(Debug, Default)]
pub struct OrderIndex {
    map: HashMap<OrderId, OrderLocation>,
}

impl OrderIndex {
    
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, order_id: OrderId, location: OrderLocation) {
        self.map.insert(order_id, location);
    }

    pub fn remove(&mut self, order_id: OrderId) -> Option<OrderLocation> {
        self.map.remove(&order_id)
    }

    pub fn contains(&self, order_id: OrderId) -> bool {
        self.map.contains_key(&order_id)
    }

}
