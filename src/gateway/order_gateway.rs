use crate::cli::parser::Command;
use crate::domain::{Order, OrderType, Side};
use crate::engine::order_book::{Price, Quantity};

#[derive(Debug)]
pub enum GatewayEvent {
    NewOrder(Order),
    Cancel { order_id: u64 },
    Reject(String),
}

#[derive(Debug)]
pub struct OrderGateway {
    next_order_id: u64,
    next_sequence: u64,
}

impl OrderGateway {
    pub fn new() -> Self {
        Self {
            next_order_id: 1,
            next_sequence: 1,
        }
    }

     pub fn process_command(&mut self, cmd: Command) -> GatewayEvent {
        match cmd {
            Command::NewOrder {
                side,
                order_type,
                price,
                quantity,
            } => self.handle_new_order(side, order_type, price, quantity),

            Command::Cancel { order_id } => GatewayEvent::Cancel { order_id },
        }
    }
}

impl OrderGateway {
    fn handle_new_order(
        &mut self,
        side: Side,
        order_type: OrderType,
        price: Option<Price>,
        quantity: Quantity,
    ) -> GatewayEvent {
        // Basic validation
        if quantity == 0 {
            return GatewayEvent::Reject("quantity must be > 0".into());
        }

        match order_type {
            OrderType::Limit => {
                if price.is_none() {
                    return GatewayEvent::Reject("limit order requires price".into());
                }
            }
            OrderType::Market => {
                if price.is_some() {
                    return GatewayEvent::Reject("market order must not have price".into());
                }
            }
        }

        let order = Order {
            id: self.next_order_id,
            side,
            order_type,
            price,
            quantity,
            sequence: self.next_sequence,
        };

        self.next_order_id += 1;
        self.next_sequence += 1;

        GatewayEvent::NewOrder(order)
    }
}

