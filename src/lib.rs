//! # TGJU Rust Client
//! 
//! A highly modular and robust asynchronous client for scraping 
//! financial data (Gold, Silver, Coins, Currencies) from tgju.org.

pub mod client;
pub mod error;
pub mod models;

// Re-exporting main structures for easier access
pub use client::TgjuClient;
pub use error::TgjuError;
pub use models::{MarketCategory, MarketItem};