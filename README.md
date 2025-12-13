# Rust-Execution-Engine
Deterministic limit order book and matching engine written in Rust, implementing price–time priority, market and limit orders, and exchange-style trade execution via a CLI interface.

# High Level Architecture
CLI Input
   ↓
Command Parser
   ↓
Order Gateway (sequencing)
   ↓
MatchingEngine
 ├── OrderBook
 │    ├── Bids: Price → FIFO queue of BUY orders
 │    └── Asks: Price → FIFO queue of SELL orders
 ├── OrderIndex (order_id → location)
 └── Matching Logic
   ↓
Trade Events (stdout)
