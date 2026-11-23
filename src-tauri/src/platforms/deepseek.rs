use super::PlatformBalance;
use crate::models::BalanceInfo;
use anyhow::{Result, Context};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct BalanceResponse {
    is_available: bool,
    balance_infos: Option<Vec<BalanceDetail>>,
}

#[derive(Debug, Deserialize)]
struct BalanceDetail {
    total_balance: String,
    granted_balance: Option<String>,
    topped_up_balance: Option<String>,
}

pub struct DeepSeek;

#[async_trait::async_trait]
impl PlatformBalance for DeepSeek {
    async fn fetch_balance(&self, api_key: &str) -> Result<BalanceInfo> {
        let client = reqwest::Client::new();
        let resp = client
            .get("https://api.deepseek.com/user/balance")
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await
            .context("Failed to call DeepSeek API")?;

        let data: BalanceResponse = resp.json().await.context("Failed to parse DeepSeek response")?;

        let total = data.balance_infos.as_ref()
            .and_then(|infos| infos.first())
            .map(|info| info.total_balance.parse::<f64>().unwrap_or(0.0))
            .unwrap_or(0.0);

        Ok(BalanceInfo {
            platform: "DeepSeek".into(),
            platform_id: "deepseek".into(),
            total_balance: total,
            used_balance: 0.0,
            remaining_balance: total,
            currency: "CNY".into(),
            available: data.is_available,
            has_api: true,
            console_url: "https://platform.deepseek.com/usage".into(),
            error: None,
        })
    }
}
