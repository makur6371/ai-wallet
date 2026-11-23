pub mod deepseek;
pub mod zhipu;
pub mod moonshot;
pub mod minimax;
pub mod baichuan;
pub mod qianfan;

use crate::models::BalanceInfo;
use anyhow::Result;

#[async_trait::async_trait]
pub trait PlatformBalance {
    async fn fetch_balance(&self, api_key: &str) -> Result<BalanceInfo>;
}

pub fn get_platform_fetcher(platform_id: &str) -> Option<Box<dyn PlatformBalance + Send + Sync>> {
    match platform_id {
        "deepseek" => Some(Box::new(deepseek::DeepSeek)),
        "zhipu" => Some(Box::new(zhipu::Zhipu)),
        "moonshot" => Some(Box::new(moonshot::Moonshot)),
        "minimax" => Some(Box::new(minimax::MiniMax)),
        "baichuan" => Some(Box::new(baichuan::Baichuan)),
        "qianfan" => Some(Box::new(qianfan::Qianfan)),
        _ => None,
    }
}

pub fn get_console_url(platform_id: &str) -> (&str, &str) {
    match platform_id {
        "deepseek" => ("https://platform.deepseek.com/usage", "DeepSeek"),
        "zhipu" => ("https://open.bigmodel.cn/usercenter/resourcepack", "智谱GLM"),
        "moonshot" => ("https://platform.moonshot.cn/console/account", "Kimi/Moonshot"),
        "minimax" => ("https://platform.minimaxi.com/user-center/basic-information", "MiniMax"),
        "baichuan" => ("https://platform.baichuan-ai.com/console/account", "百川"),
        "qianfan" => ("https://console.bce.baidu.com/qianfan/overview", "百度千帆"),
        "dashscope" => ("https://bailian.console.aliyun.com/", "通义千问"),
        "xinghuo" => ("https://console.xfyun.cn/console/billing", "讯飞星火"),
        "yi" => ("https://platform.lingyiwanwu.com/billing", "零一万物"),
        _ => ("", "Unknown"),
    }
}
