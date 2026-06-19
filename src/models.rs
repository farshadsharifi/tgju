/// دسته‌بندی اصلی بازار برای مشخص کردن آدرس URL هدف
pub enum MarketCategory {
    Gold,
    Coin,
    Currency,
}

impl MarketCategory {
    /// برگرداندن آدرس اینترنتی بر اساس دسته‌بندی
    pub fn url(&self) -> &'static str {
        match self {
            MarketCategory::Gold => "https://www.tgju.org/gold-chart",
            MarketCategory::Coin => "https://www.tgju.org/coin",
            MarketCategory::Currency => "https://www.tgju.org/currency",
        }
    }
}

/// شناسه‌های بازار (شما می‌توانید بقیه ارزها را به این الگو اضافه کنید)
pub enum MarketItem {
    // --- طلا ---
    Gold18,          // طلای 18 عیار / 750
    Gold18_740,      // طلای 18 عیار / 740
    Gold24,          // طلای 24 عیار
    GoldUsed,        // طلای دست دوم
    Mesghal,         // مثقال طلا
    GoldFutures,     // آبشده نقدی
    
    // --- نقره ---
    Silver925,       // گرم نقره ۹۲۵
    Silver999,       // گرم نقره ۹۹۹
    
    // --- سکه ---
    CoinEmami,       // سکه امامی
    CoinBahar,       // سکه بهار آزادی
    CoinHalf,        // نیم سکه
    CoinQuarter,     // ربع سکه
    CoinGram,        // سکه گرمی
    
    // --- ارز ---
    Dollar,          // دلار
    Euro,            // یورو
    // بقیه ارزها را می‌توانید اینجا اضافه کنید...
}

impl MarketItem {
    /// برگرداندن شناسه (Slug) مربوط به هر آیتم
    pub fn slug(&self) -> &'static str {
        match self {
            MarketItem::Gold18 => "geram18",
            MarketItem::Gold18_740 => "gold_740k",
            MarketItem::Gold24 => "geram24",
            MarketItem::GoldUsed => "gold_mini_size",
            MarketItem::Mesghal => "mesghal",
            MarketItem::GoldFutures => "gold_futures",
            MarketItem::Silver925 => "silver_925",
            MarketItem::Silver999 => "silver_999",
            MarketItem::CoinEmami => "sekee",
            MarketItem::CoinBahar => "sekeb",
            MarketItem::CoinHalf => "nim",
            MarketItem::CoinQuarter => "rob",
            MarketItem::CoinGram => "gerami",
            MarketItem::Dollar => "price_dollar_rl",
            MarketItem::Euro => "price_eur",
        }
    }

    /// تشخیص اینکه این آیتم متعلق به کدام صفحه است
    pub fn category(&self) -> MarketCategory {
        match self {
            MarketItem::Gold18 | MarketItem::Gold18_740 | MarketItem::Gold24 |
            MarketItem::GoldUsed | MarketItem::Mesghal | MarketItem::GoldFutures |
            MarketItem::Silver925 | MarketItem::Silver999 => MarketCategory::Gold,
            
            MarketItem::CoinEmami | MarketItem::CoinBahar | MarketItem::CoinHalf |
            MarketItem::CoinQuarter | MarketItem::CoinGram => MarketCategory::Coin,
            
            MarketItem::Dollar | MarketItem::Euro => MarketCategory::Currency,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_item_slugs() {
        // تست بررسی صحت نگاشت آیتم‌ها به اسلاگ‌های متناظر
        assert_eq!(MarketItem::Gold18.slug(), "geram18");
        assert_eq!(MarketItem::Dollar.slug(), "price_dollar_rl");
        assert_eq!(MarketItem::CoinEmami.slug(), "sekee");
    }

    #[test]
    fn test_market_category_mapping() {
        // تست بررسی اینکه هر آیتم به دسته بندی درستی هدایت می‌شود
        match MarketItem::Gold18.category() {
            MarketCategory::Gold => assert!(true),
            _ => panic!("Gold18 should belong to MarketCategory::Gold"),
        }

        match MarketItem::Dollar.category() {
            MarketCategory::Currency => assert!(true),
            _ => panic!("Dollar should belong to MarketCategory::Currency"),
        }
    }
}