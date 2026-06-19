# TGJU Rust Client 🦀💸

[![Crates.io](https://img.shields.io/crates/v/tgju.svg)](https://crates.io/crates/tgju)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**TGJU Rust Client** is a fast, asynchronous, and robust web scraper library written in Rust to fetch real-time financial data, including **Gold, Silver, Coins, and Currencies** from [tgju.org](https://www.tgju.org). 

Developed by **Farshad Sharifi** ([farshadsharifi.ir](https://farshadsharifi.ir) | [GitHub](https://github.com/farshadsharifi)).

---

## 🇮🇷 راهنمای فارسی (Persian Documentation)

این کتابخانه به شما اجازه می‌دهد تا به ساده‌ترین شکل ممکن، قیمت‌های لحظه‌ای طلا، نقره، سکه و ارز را در بک‌اند پروژه‌های Rust خود (مثل سرورهای Actix یا Axum) دریافت کنید. این کتابخانه کاملاً Asynchronous است و از پکیج قدرتمند `scraper` به جای عبارات منظم (Regex) برای پایداری بیشتر در برابر تغییرات سایت مبدا استفاده می‌کند.

### ویژگی‌ها:
- معماری کاملاً ماژولار و Asynchronous.
- مدیریت خطای حرفه‌ای (Error Handling).
- استخراج دقیق قیمت‌ها بر اساس `data-market-nameslug`.
- عدم وابستگی به API (استفاده از Web Scraping امن).

### روش استفاده (Usage)

ابتدا کتابخانه را به پروژه خود اضافه کنید:
```bash
cargo add tgju tokio

سپس در کدهای خود به این شکل استفاده کنید:
use tgju::{TgjuClient, MarketItem};
use tokio;

#[tokio::main]
async fn main() {
    // ایجاد یک نمونه جدید از کلاینت
    let client = TgjuClient::new();

    // دریافت قیمت طلای 18 عیار
    match client.get_price(MarketItem::Gold18).await {
        Ok(price) => println!("قیمت طلای ۱۸ عیار: {} ریال", price),
        Err(e) => eprintln!("خطا در دریافت قیمت: {}", e),
    }

    // دریافت قیمت دلار
    match client.get_price(MarketItem::Dollar).await {
        Ok(price) => println!("قیمت دلار: {} ریال", price),
        Err(e) => eprintln!("خطا: {}", e),
    }
}

🇬🇧 English Documentation

A professional asynchronous wrapper to scrape prices from TGJU. It parses the DOM safely and handles network anomalies smoothly.
Quick Start

Add to your Cargo.toml:
[dependencies]
tgju = "0.1.0"
tokio = { version = "1", features = ["full"] }

Example:
use tgju::{TgjuClient, MarketItem};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TgjuClient::new();
    
    // Fetch 18-karat Gold price
    let gold_price = client.get_price(MarketItem::Gold18).await?;
    println!("18k Gold Price: {}", gold_price);
    
    // Fetch Euro price
    let euro_price = client.get_price(MarketItem::Euro).await?;
    println!("Euro Price: {}", euro_price);

    Ok(())
}

Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Visit my website: farshadsharifi.ir