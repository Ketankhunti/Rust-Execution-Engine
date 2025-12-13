use std::collections::{BTreeMap, VecDeque};

pub type Price = i64;
pub type Quantity = i64;

#[derive(Default, Debug)]
pub struct OrderBook{
    bids: BTreeMap<Price, VecDeque<Order>>,  // BUY side (highest price is best)
    asks: BTreeMap<Price, VecDeque<Order>>,  // SELL side (lowest price is best)
}

impl OrderBook {

    pub fn best_bid_price(&self) -> Option<Price> {
        self.bids.keys().next_back().copied()
    }

    pub fn best_ask_price(&self) -> Option<Price> {
        self.asks.keys().next().copied()
    }

    pub fn insert(&mut self, order: Order) {
        let book_side = match order.side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        book_side.
            entry(order.price)
            .or_insert_with(VecDeque::new)
            .push_back(order);
    }

    pub fn peek_best_bid(&self) -> Option<&Order> {
        self.bids
            .iter()
            .next_back()
            .and_then(|(_,queue)| queue.front())
    }

    pub fn peek_best_ask(&self) -> Option<&Order> {
        self.bids
            .iter()
            .next()
            .and_then(|(_,queue)| queue.front())
    }

    pub fn pop_best_bid(&mut self) -> Option(Order) {
        let price = self.best_bid_price()?;
        let queue = self.bids.get_mut(&price)?;
        let order = queue.pop_front();

        if queue.is_empty() {
            self.bids.remove(&price);
        }

        order
    }

    pub fn pop_best_ask(&mut self) -> Option<Order> {
        let price = self.best_ask_price()?;
        let queue = self.asks.get_mut(&price)?;
        let order = queue.pop_front();

        if queue.is_empty() {
            self.asks.remove(&price);
        }

        order
    }

    pub fn reduce_best_price(&mut self, qty: Quantity) {
        if let Some(price) = self.best_bid_price() {
            if let Some(queue) = self.bids.get_mut(&price) {
                if let Some(order) = queue.front_mut() {
                    if order.quantity <= qty {
                        queue.pop_front();
                    }
                    else {
                        order.quantity -= qty;
                    }
                }
                if queue.is_empty() {
                    self.bids.remove(&price);
                }
            }
        }
    }

    pub fn reduce_best_ask(&mut self, qty: Quantity) {
        if let Some(price) = self.best_ask_price() {
            if let Some(queue) = self.asks.get_mut(&price) {
                if let Some(order) = queue.front_mut() {
                    if order.quantity <= qty {
                        queue.pop_front();
                    }
                    else {
                        order.quantity -= qty;
                    }
                }
                if queue.is_empty() {
                    self.asks.remove(&price);
                }
            }
        }
    }
}