use super::PlatformBalance;
use crate::models::BalanceInfo;
use anyhow::{Result, Context};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct UserResponse {
    data: Option<UserData>,
}

#[derive(Debug, Deserialize)]
struct UserData {
    quota: Option<f64>,
    #[serde(rename = "usedQuota")]
    used_quota: Option<f64>,
    #[serde(rename = "remainQuota")]
    remain_quota: Option<f64>,
}

pub struct Baichuan;

#[async_trait::async_trait]
impl PlatformBalance for Baichuan {
    async fn fetch_balance(&self, api_key: &str) -> Result<BalanceInfo> {
        let client = reqwest::Client::new();
        let resp = client
            .get("https://api.baichuan-ai.com/v1/user/info")
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await
            .context("Failed to call Baichuan API")?;

        let data: UserResponse = resp.json().await.context("Failed to parse Baichuan response")?;
        let user = data.data.unwrap_or(UserData { quota: None, used_quota: None, remain_quota: None });

        Ok(BalanceInfo {
            platform: "百川".into(),
            platform_id: "baichuan".into(),
            total_balance: user.quota.unwrap_or(0.0),
            used_balance: user.used_quota.unwrap_or(0.0),
            remaining_balance: user.remain_quota.unwrap_or(0.0),
            currency: "CNY".into(),
            available: user.remain_quota.unwrap_or(0.0) > 0.0,
            has_api: true,
            console_url: "https://platform.baichuan-ai.com/console/account".into(),
            error: None,
        })
    }
}
