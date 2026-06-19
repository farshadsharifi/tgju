use tgju::{TgjuClient, MarketItem};

/// این تست به صورت واقعی به سرور متصل شده و دریافت قیمت طلا را بررسی می‌کند
#[tokio::test]
async fn test_fetch_live_gold_price() {
    let client = TgjuClient::new();
    
    // تلاش برای دریافت قیمت طلای ۱۸ عیار
    let result = client.get_price(MarketItem::Gold18).await;
    
    // بررسی اینکه آیا درخواست بدون خطا انجام شده است
    assert!(result.is_ok(), "تست اتصال به بخش طلا با خطا مواجه شد");
    
    let price = result.unwrap();
    // قیمت نباید خالی باشد
    assert!(!price.is_empty(), "قیمت استخراج شده خالی است");
    println!("Live Test - Gold 18k Price: {}", price);
}

/// این تست دریافت قیمت ارز (دلار) را بررسی می‌کند
#[tokio::test]
async fn test_fetch_live_currency_price() {
    let client = TgjuClient::new();
    
    // تلاش برای دریافت قیمت دلار
    let result = client.get_price(MarketItem::Dollar).await;
    
    assert!(result.is_ok(), "تست اتصال به بخش ارز با خطا مواجه شد");
    
    let price = result.unwrap();
    assert!(!price.is_empty());
    println!("Live Test - Dollar Price: {}", price);
}

/// این تست دریافت قیمت سکه را بررسی می‌کند
#[tokio::test]
async fn test_fetch_live_coin_price() {
    let client = TgjuClient::new();
    
    // تلاش برای دریافت قیمت سکه امامی
    let result = client.get_price(MarketItem::CoinEmami).await;
    
    assert!(result.is_ok(), "تست اتصال به بخش سکه با خطا مواجه شد");
    
    let price = result.unwrap();
    assert!(!price.is_empty());
    println!("Live Test - Emami Coin Price: {}", price);
}