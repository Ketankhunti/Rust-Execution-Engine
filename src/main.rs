use std::io::{self, BufRead};

use rust_matching_engine::cli::parser::parse_command;
use rust_matching_engine::gateway::order_gateway::{OrderGateway};
use rust_matching_engine::engine::matching_engine::{EngineEvent, MatchingEngine};


fn main() {
    let stdin = io::stdin();
    let mut gateway = OrderGateway::new();
    let mut engine = MatchingEngine::new();

    println!("Matching engine started. Enter commands:");

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Failed to read line: {}", e);
                continue;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        // 1️⃣ Parse CLI input
        let command = match parse_command(&line) {
            Ok(cmd) => cmd,
            Err(err) => {
                eprintln!("Parse error: {}", err);
                continue;
            }
        };

        // 2️⃣ Send to gateway
        let gateway_event = gateway.process_command(command);

        // 3️⃣ Send to matching engine
        let engine_events = engine.on_event(gateway_event);

        // 4️⃣ Print engine output
        for event in engine_events {
            match event {
                EngineEvent::Trade(trade) => {
                    println!(
                        "TRADE price={} qty={} buy_id={} sell_id={}",
                        trade.price,
                        trade.quantity,
                        trade.buy_order_id,
                        trade.sell_order_id
                    );
                }
                EngineEvent::Reject(reason) => {
                    eprintln!("REJECT: {}", reason);
                }
            }
        }
    }
}
