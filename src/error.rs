use thiserror::Error;

/// خطاهای سفارشی کتابخانه TGJU
#[derive(Error, Debug)]
pub enum TgjuError {
    /// خطاهای مربوط به شبکه و کانکشن
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    /// خطایی که در صورت یافت نشدن اسلاگ در صفحه رخ می‌دهد
    #[error("Price for slug '{0}' not found in the DOM")]
    NotFound(String),
    
    /// خطای مربوط به تغییر ساختار HTML سایت مبدا
    #[error("Failed to parse HTML structure")]
    ParseError,
}