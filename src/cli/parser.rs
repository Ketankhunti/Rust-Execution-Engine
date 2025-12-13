use crate::domain::order::{OrderId, OrderType,Side};
use crate::engine::order_book::{Price, Quantity};

pub enum Command {
    NewOrder {
        side: Side,
        order_type: OrderType,
        price: Option<Price>,
        quantity: Quantity,
    },
    Cancel {
        order_id: OrderId,
    },
}

pub fn parse_command(input: &str) -> Result<Command, String> {

    let parts: Vec<&str> = input.trim().split_ascii_whitespace().collect();

    if parts.is_empty() {
        return Err("Empty command".to_string());
    }

    match parts[0].to_uppercase().as_str() {
        "BUY" | "SELL" => {
            parse_new_order(&parts)
        },
        "CANCEL" => {
            parse_cancel(&parts)
        },
        _ => Err("Unknown command".to_string()),
    }
}

fn parse_new_order(parts: &[&str]) -> Result<Command, String> {
    if parts.len() < 3 {
        return Err("Invalid order foramt".into());
    }

    let side = match parts[0].to_uppercase().as_str() {
        "BUY" => Side::Buy,
        "SELL" => Side::Sell,
        _ => return Err("Invalid side".into()),
    };

    let order_type = match parts[1].to_uppercase().as_str() {
        "LIMIT" => OrderType::Limit,
        "MARKET" => OrderType::Market,
        _ => return Err("Invalid order type".into()),
    };

    match order_type {
        OrderType::Limit => {
            if parts.len() != 4 {
                return Err("Invalid limit order format".into());
            }

            let price: Price = parts[2]
                .parse()
                .map_err(|_| "invalid price".to_string())?;

            let quantity: Quantity = parts[3]
                .parse()
                .map_err(|_| "invalid quantity".to_string())?;

            Ok(Command::NewOrder {
                side,
                order_type,
                price: Some(price),
                quantity,
            })
        }
        OrderType::Market => {
            if parts.len() != 3 {
                return Err("MARKET order requires quantity only".into());
            }

            let quantity: Quantity = parts[2]
                .parse()
                .map_err(|_| "invalid quantity".to_string())?;

            Ok(Command::NewOrder {
                side,
                order_type,
                price: None,
                quantity,
            })
        }
    }
    
}

fn parse_cancel(parts: &[&str]) -> Result<Command, String> {
    if parts.len() != 2 {
        return Err("CANCEL requires order_id".into());
    }

    let order_id: u64 = parts[1]
        .parse()
        .map_err(|_| "invalid order id".to_string())?;

    Ok(Command::Cancel { order_id })
}
