use crate::error::TgjuError;
use crate::models::{MarketItem};
use scraper::{Html, Selector};

/// کلاینت اصلی برای ارتباط با سایت TGJU
pub struct TgjuClient {
    http_client: reqwest::Client,
}

impl TgjuClient {
    /// ساخت یک نمونه جدید از کلاینت
    pub fn new() -> Self {
        Self {
            // ساخت یک کلاینت با تنظیمات پیش‌فرض (مثل تنظیم User-Agent برای جلوگیری از بلاک شدن)
            http_client: reqwest::Client::builder()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36")
                .build()
                .unwrap_or_default(),
        }
    }

    /// دریافت قیمت یک آیتم مشخص (مثلا فقط طلای ۱۸ عیار)
    pub async fn get_price(&self, item: MarketItem) -> Result<String, TgjuError> {
        let category = item.category();
        let url = category.url();
        let slug = item.slug();

        // ۱. دریافت کدهای HTML صفحه
        let html_content = self.http_client.get(url).send().await?.text().await?;

        // ۲. تبدیل رشته HTML به ساختار درختی قابل جستجو
        let document = Html::parse_document(&html_content);

        // ۳. ساخت سلکتور CSS (دقیقا به دنبال تگ tr با شناسه مورد نظر می‌گردیم)
        let selector_str = format!("tr[data-market-nameslug='{}']", slug);
        let row_selector = Selector::parse(&selector_str)
            .map_err(|_| TgjuError::ParseError)?;

        // ۴. جستجو در صفحه و استخراج اتریبیوت data-price
        if let Some(row) = document.select(&row_selector).next() {
            if let Some(price) = row.value().attr("data-price") {
                return Ok(price.to_string());
            }
        }

        // اگر اسلاگ پیدا نشد، خطای اختصاصی برمی‌گردانیم
        Err(TgjuError::NotFound(slug.to_string()))
    }
}

impl Default for TgjuClient {
    fn default() -> Self {
        Self::new()
    }
}