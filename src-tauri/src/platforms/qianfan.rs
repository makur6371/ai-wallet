use super::PlatformBalance;
use crate::models::BalanceInfo;
use anyhow::{Result, Context};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct BalanceResponse {
    error_code: Option<i64>,
    error_msg: Option<String>,
    result: Option<BalanceResult>,
}

#[derive(Debug, Deserialize)]
struct BalanceResult {
    total_quota: Option<f64>,
    used_quota: Option<f64>,
    remain_quota: Option<f64>,
}

pub struct Qianfan;

#[async_trait::async_trait]
impl PlatformBalance for Qianfan {
    async fn fetch_balance(&self, api_key: &str) -> Result<BalanceInfo> {
        let client = reqwest::Client::new();
        let resp = client
            .get("https://qianfan.baidubce.com/v2/user/info")
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await
            .context("Failed to call Qianfan API")?;

        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();

        if !status.is_success() {
            return Err(anyhow::anyhow!("Qianfan API error ({}): {}", status, text));
        }

        let data: BalanceResponse = serde_json::from_str(&text)
            .context(format!("Failed to parse Qianfan response: {}", text))?;

        if let Some(code) = data.error_code {
            if code != 0 {
                return Err(anyhow::anyhow!("Qianfan API error ({}): {}", code, data.error_msg.unwrap_or_default()));
            }
        }

        let result = data.result.unwrap_or(BalanceResult {
            total_quota: None,
            used_quota: None,
            remain_quota: None,
        });

        Ok(BalanceInfo {
            platform: "百度千帆".into(),
            platform_id: "qianfan".into(),
            total_balance: result.total_quota.unwrap_or(0.0),
            used_balance: result.used_quota.unwrap_or(0.0),
            remaining_balance: result.remain_quota.unwrap_or(0.0),
            currency: "CNY".into(),
            available: result.remain_quota.unwrap_or(0.0) > 0.0,
            has_api: true,
            console_url: "https://console.bce.baidu.com/qianfan/overview".into(),
            error: None,
        })
    }
}
