use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceInfo {
    pub platform: String,
    pub platform_id: String,
    pub total_balance: f64,
    pub used_balance: f64,
    pub remaining_balance: f64,
    pub currency: String,
    pub available: bool,
    pub has_api: bool,
    pub console_url: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformConfig {
    pub id: String,
    pub name: String,
    pub api_key: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub platforms: Vec<PlatformConfig>,
    pub refresh_interval: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            platforms: vec![
                PlatformConfig { id: "deepseek".into(), name: "DeepSeek".into(), api_key: String::new(), enabled: true },
                PlatformConfig { id: "zhipu".into(), name: "智谱GLM".into(), api_key: String::new(), enabled: true },
                PlatformConfig { id: "moonshot".into(), name: "Kimi/Moonshot".into(), api_key: String::new(), enabled: true },
                PlatformConfig { id: "minimax".into(), name: "MiniMax".into(), api_key: String::new(), enabled: true },
                PlatformConfig { id: "baichuan".into(), name: "百川".into(), api_key: String::new(), enabled: true },
                PlatformConfig { id: "qianfan".into(), name: "百度千帆".into(), api_key: String::new(), enabled: true },
                PlatformConfig { id: "dashscope".into(), name: "通义千问".into(), api_key: String::new(), enabled: true },
                PlatformConfig { id: "xinghuo".into(), name: "讯飞星火".into(), api_key: String::new(), enabled: true },
                PlatformConfig { id: "yi".into(), name: "零一万物".into(), api_key: String::new(), enabled: true },
            ],
            refresh_interval: 300,
        }
    }
}
