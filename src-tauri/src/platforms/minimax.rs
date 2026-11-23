use super::PlatformBalance;
use crate::models::BalanceInfo;
use anyhow::{Result, Context};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct BalanceResponse {
    total_balance: Option<f64>,
    used_balance: Option<f64>,
    remaining_balance: Option<f64>,
}

pub struct MiniMax;

#[async_trait::async_trait]
impl PlatformBalance for MiniMax {
    async fn fetch_balance(&self, api_key: &str) -> Result<BalanceInfo> {
        let client = reqwest::Client::new();
        let resp = client
            .get("https://api.minimax.chat/v1/user/balance")
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await
            .context("Failed to call MiniMax API")?;

        let data: BalanceResponse = resp.json().await.context("Failed to parse MiniMax response")?;

        Ok(BalanceInfo {
            platform: "MiniMax".into(),
            platform_id: "minimax".into(),
            total_balance: data.total_balance.unwrap_or(0.0),
            used_balance: data.used_balance.unwrap_or(0.0),
            remaining_balance: data.remaining_balance.unwrap_or(0.0),
            currency: "CNY".into(),
            available: data.remaining_balance.unwrap_or(0.0) > 0.0,
            has_api: true,
            console_url: "https://platform.minimaxi.com/user-center/basic-information".into(),
            error: None,
        })
    }
}
