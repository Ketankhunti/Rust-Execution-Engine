use crate::{domain::{Order, OrderType, Side, trade::Trade}, engine::{order_book::OrderBook, order_index::{OrderIndex, OrderLocation}}, gateway::order_gateway::GatewayEvent};


#[derive(Debug)]
pub enum EngineEvent{
    Trade(Trade),
    Reject(String),
}

#[derive(Debug)]
pub struct MatchingEngine {
    order_book: OrderBook,
    order_index: OrderIndex,
}

impl MatchingEngine {
    pub fn new() -> Self {
        Self {
            order_book: OrderBook::default(),
            order_index: OrderIndex::new(),
        }
    }

    pub fn on_event(&mut self, event: GatewayEvent) -> Vec<EngineEvent> {
        match event {
            GatewayEvent::NewOrder(order) => {
                self.handle_new_order(order)
            },
            GatewayEvent::Cancel { order_id } => {
                self.handle_cancel(order_id)
            },
            GatewayEvent::Reject(reason) => {
                vec![EngineEvent::Reject(reason)]
            }
        }
    }

    fn handle_new_order(&mut self, mut order: Order) -> Vec<EngineEvent> {
        let mut events = Vec::new();

        match order.side {
            Side::Buy => {
                self.match_buy_order(&mut order, &mut events);
            }
            Side::Sell => {
                self.match_sell_order(&mut order, &mut events);
            }
        }

        events

    }

    fn match_buy_order(&mut self, order: &mut Order, events: &mut Vec<EngineEvent>) {
        loop {
            let bestask_price = match self.order_book.best_ask_price() {
                Some(price) => price,
                None => break, // no liquidity
            };

            // Check price constraint for LIMIT orders
            if order.order_type == OrderType::Limit {
                if let Some(order_price) = order.price {
                    if order_price < bestask_price {
                        break; // price too high
                    }
                }
            }

            // Get resting sell order
            let mut resting = match self.order_book.pop_best_ask() {
                Some(o) => o,
                None => break, // no resting order
            };

            let traded_qty = order.quantity.min(resting.quantity);

            // Emit trade
            events.push(EngineEvent::Trade(Trade {
                price: bestask_price,
                quantity: traded_qty,
                buy_order_id: order.id,
                sell_order_id: resting.id
            }));

            order.quantity -= traded_qty;
            resting.quantity -= traded_qty;

            if resting.quantity > 0 {
                // Reinsert partially filled resting order
                self.order_book.insert(resting);
            }
            else if resting.quantity == 0 {
                self.order_index.remove(resting.id);
            }


            if order.quantity == 0 {
                // Fully filled incoming order
                return;
            }

            self.insert_if_remaining(order.clone());

        }
    }

    fn match_sell_order(&mut self, order: &mut Order, events: &mut Vec<EngineEvent>) {
        loop {
            let best_bid_price = match self.order_book.best_bid_price() {
                Some(p) => p,
                None => break, // no liquidity
            };

            // Price constraint for LIMIT orders
            if order.order_type == OrderType::Limit {
                if let Some(limit_price) = order.price {
                    if best_bid_price < limit_price {
                        break;
                    }
            }

            }

            // Get resting BUY order
            let mut resting = match self.order_book.pop_best_bid() {
                Some(o) => o,
                None => break,
            };

            let traded_qty = order.quantity.min(resting.quantity);

            // Emit trade (price = resting bid price)
            events.push(EngineEvent::Trade(Trade {
                price: best_bid_price,
                quantity: traded_qty,
                buy_order_id: resting.id,
                sell_order_id: order.id,
            }));

            // Update quantities
            order.quantity -= traded_qty;
            resting.quantity -= traded_qty;

            // Reinsert resting order if partially filled
            if resting.quantity > 0 {
                self.order_book.insert(resting);
            }
            else if resting.quantity == 0 {
                self.order_index.remove(resting.id);
            }

            // Incoming SELL fully filled
            if order.quantity == 0 {
                break;
            }
        }

        // Insert remaining SELL LIMIT orders only
        self.insert_if_remaining(order.clone());

    }

    fn insert_if_remaining(&mut self, order: Order) {
        if order.quantity == 0 {
            return;
        }

        if order.order_type == OrderType::Limit {
            let price = order.price.expect("limit order must have price");

            self.order_book.insert(order.clone());
            self.order_index.insert(
                order.id,
                OrderLocation {
                    side: order.side,
                    price,
                },
            );
        }
    }
    
    fn handle_cancel(&mut self, order_id: u64) -> Vec<EngineEvent> {
        if let Some(location) = self.order_index.remove(order_id) {
            let removed = self
                .order_book
                .remove_order(location.side, location.price, order_id);

            if removed {
                Vec::new()
            } else {
                vec![EngineEvent::Reject("order not found in book".into())]
            }
        } else {
            vec![EngineEvent::Reject("order not found".into())]
        }
    }
}

