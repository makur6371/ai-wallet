use super::PlatformBalance;
use crate::models::BalanceInfo;
use anyhow::{Result, Context};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct BalanceResponse {
    code: Option<i64>,
    msg: Option<String>,
    data: Option<BalanceData>,
}

#[derive(Debug, Deserialize)]
struct BalanceData {
    balance: Option<f64>,
    cash_balance: Option<f64>,
    present_balance: Option<f64>,
    available_balance: Option<f64>,
}

pub struct Zhipu;

#[async_trait::async_trait]
impl PlatformBalance for Zhipu {
    async fn fetch_balance(&self, api_key: &str) -> Result<BalanceInfo> {
        let client = reqwest::Client::new();
        let resp = client
            .get("https://open.bigmodel.cn/api/paas/v4/account/balance")
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await
            .context("Failed to call Zhipu API")?;

        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();

        if !status.is_success() {
            return Err(anyhow::anyhow!("Zhipu API error ({}): {}", status, text));
        }

        let data: BalanceResponse = serde_json::from_str(&text)
            .context(format!("Failed to parse Zhipu response: {}", text))?;

        if let Some(code) = data.code {
            if code != 0 {
                return Err(anyhow::anyhow!("Zhipu API error ({}): {}", code, data.msg.unwrap_or_default()));
            }
        }

        let balance_data = data.data.unwrap_or(BalanceData {
            balance: None,
            cash_balance: None,
            present_balance: None,
            available_balance: None,
        });

        let remaining = balance_data.available_balance
            .or(balance_data.balance)
            .unwrap_or(0.0);
        let total = balance_data.cash_balance.unwrap_or(0.0) + balance_data.present_balance.unwrap_or(0.0);

        Ok(BalanceInfo {
            platform: "智谱GLM".into(),
            platform_id: "zhipu".into(),
            total_balance: total,
            used_balance: (total - remaining).max(0.0),
            remaining_balance: remaining,
            currency: "CNY".into(),
            available: remaining > 0.0,
            has_api: true,
            console_url: "https://open.bigmodel.cn/usercenter/resourcepack".into(),
            error: None,
        })
    }
}
