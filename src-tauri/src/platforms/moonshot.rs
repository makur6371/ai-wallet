use super::PlatformBalance;
use crate::models::BalanceInfo;
use anyhow::{Result, Context};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct BalanceResponse {
    available_balance: Option<f64>,
    voucher_balance: Option<f64>,
    cash_balance: Option<f64>,
}

pub struct Moonshot;

#[async_trait::async_trait]
impl PlatformBalance for Moonshot {
    async fn fetch_balance(&self, api_key: &str) -> Result<BalanceInfo> {
        let client = reqwest::Client::new();
        let resp = client
            .get("https://api.moonshot.cn/v1/user/balance")
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await
            .context("Failed to call Moonshot API")?;

        let data: BalanceResponse = resp.json().await.context("Failed to parse Moonshot response")?;

        let remaining = data.available_balance.unwrap_or(0.0);
        let total = data.cash_balance.unwrap_or(0.0) + data.voucher_balance.unwrap_or(0.0);

        Ok(BalanceInfo {
            platform: "Kimi/Moonshot".into(),
            platform_id: "moonshot".into(),
            total_balance: total,
            used_balance: total - remaining,
            remaining_balance: remaining,
            currency: "CNY".into(),
            available: remaining > 0.0,
            has_api: true,
            console_url: "https://platform.moonshot.cn/console/account".into(),
            error: None,
        })
    }
}
