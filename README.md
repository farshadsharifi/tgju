# TGJU Rust Client 🦀💸

<div align="center">

[![Crates.io](https://img.shields.io/crates/v/tgju.svg)](https://crates.io/crates/tgju)
[![Rust](https://img.shields.io/badge/Rust-Async-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![GitHub Stars](https://img.shields.io/github/stars/farshadsharifi/tgju)](https://github.com/farshadsharifi/tgju)

### Fast, Async & Reliable Financial Market Data Scraper for Rust

Fetch real-time prices of **Gold, Silver, Coins, and Currencies** directly from **TGJU** using a fully asynchronous Rust client.

**Designed for production-ready Rust backends including Axum, Actix Web, Rocket, and Tokio-based services.**

🌐 Website: https://farshadsharifi.ir  
📦 Crate: https://crates.io/crates/tgju  
🔗 Repository: https://github.com/farshadsharifi/tgju

</div>

---

## ✨ Features

- 🚀 Fully asynchronous architecture powered by Tokio
- 🦀 Written in pure Rust
- 🔍 Robust DOM parsing using `scraper`
- ⚡ Fast and lightweight
- 🛡️ Strong error handling
- 📈 Real-time market data retrieval
- 💰 Supports Gold, Silver, Coins and Currency prices
- 🔄 No external API dependency
- 🎯 Stable extraction using `data-market-nameslug`
- 🏗️ Ready for production environments

---

## 📦 Installation

Add the crate to your project:

```bash
cargo add tgju tokio
```

Or manually update your `Cargo.toml`:

```toml
[dependencies]
tgju = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

---

# 🇮🇷 مستندات فارسی

کتابخانه **TGJU Rust Client** راهکاری ساده، سریع و پایدار برای دریافت قیمت‌های لحظه‌ای بازار ایران از وب‌سایت TGJU است.

این کتابخانه برای استفاده در پروژه‌های Backend مبتنی بر Rust طراحی شده و می‌تواند در کنار فریمورک‌هایی مانند:

- Axum
- Actix Web
- Rocket
- Warp

استفاده شود.

برخلاف بسیاری از پروژه‌های مشابه، این کتابخانه به API وابسته نیست و داده‌ها را مستقیماً از ساختار HTML سایت استخراج می‌کند. همچنین برای افزایش پایداری در برابر تغییرات سایت، از کتابخانه `scraper` به جای Regex استفاده شده است.

### بازارهای پشتیبانی‌شده

- طلا
- نقره
- سکه
- ارز

---

## 🚀 شروع سریع

```rust
use tgju::{TgjuClient, MarketItem};

#[tokio::main]
async fn main() {
    let client = TgjuClient::new();

    match client.get_price(MarketItem::Gold18).await {
        Ok(price) => println!("قیمت طلای ۱۸ عیار: {} ریال", price),
        Err(e) => eprintln!("خطا در دریافت قیمت: {}", e),
    }

    match client.get_price(MarketItem::Dollar).await {
        Ok(price) => println!("قیمت دلار: {} ریال", price),
        Err(e) => eprintln!("خطا در دریافت قیمت: {}", e),
    }
}
```

---

# 🇬🇧 English Documentation

## Overview

**TGJU Rust Client** is a professional asynchronous web scraper library for retrieving live financial market prices from TGJU.

The library safely parses the HTML DOM and extracts prices using stable market identifiers instead of relying on fragile regular expressions.

Perfect for:

- Financial dashboards
- Trading tools
- Telegram bots
- REST APIs
- Monitoring systems
- Rust backend services

---

## Quick Start

```rust
use tgju::{TgjuClient, MarketItem};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TgjuClient::new();

    let gold_price = client.get_price(MarketItem::Gold18).await?;
    println!("18K Gold Price: {}", gold_price);

    let euro_price = client.get_price(MarketItem::Euro).await?;
    println!("Euro Price: {}", euro_price);

    Ok(())
}
```

---

## Supported Market Items

Examples of available market types:

```rust
MarketItem::Gold18
MarketItem::Dollar
MarketItem::Euro
```

> Additional market items may be added in future releases.

---

## Error Handling

The library provides structured error handling for:

- Network failures
- Invalid responses
- HTML parsing errors
- Missing market entries
- Unexpected website changes

Example:

```rust
match client.get_price(MarketItem::Dollar).await {
    Ok(price) => println!("Price: {}", price),
    Err(error) => eprintln!("Failed to fetch price: {}", error),
}
```

---

## Why TGJU Rust Client?

### Traditional Approach

- Fragile Regex parsing
- Poor error handling
- Blocking requests
- Difficult maintenance

### TGJU Rust Client

✅ Async-first architecture

✅ DOM-based extraction

✅ Better resilience against HTML changes

✅ Clean and idiomatic Rust API

✅ Production-ready error handling

---

## Roadmap

- [ ] Batch price fetching
- [ ] Historical data support
- [ ] Caching layer
- [ ] Custom HTTP client configuration
- [ ] Serde integration
- [ ] WebSocket support (if available)

---

## Contributing

Contributions are welcome!

If you would like to contribute:

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to your branch
5. Open a Pull Request

For major changes, please open an issue first to discuss what you would like to change.

---

## Author

### Farshad Sharifi

🌐 Website: https://farshadsharifi.ir

🐙 GitHub: https://github.com/farshadsharifi

📦 Project Repository:

https://github.com/farshadsharifi/tgju

---

## Disclaimer

This project is an independent open-source scraper and is not affiliated with TGJU.

Please use responsibly and respect the target website's terms of service.

---

## License

Released under the MIT License.

See the [LICENSE](LICENSE) file for details.

---

<div align="center">

Made with ❤️ and Rust 🦀 by Farshad Sharifi

</div>
