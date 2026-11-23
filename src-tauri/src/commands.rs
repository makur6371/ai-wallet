use crate::config;
use crate::models::{AppConfig, BalanceInfo};
use crate::platforms::{get_console_url, get_platform_fetcher};

#[tauri::command]
pub async fn fetch_all_balances() -> Vec<BalanceInfo> {
    let app_config = config::load_config();
    let mut results = Vec::new();

    for platform in &app_config.platforms {
        if !platform.enabled {
            continue;
        }

        let info = if platform.api_key.is_empty() {
            let (url, name) = get_console_url(&platform.id);
            BalanceInfo {
                platform: name.to_string(),
                platform_id: platform.id.clone(),
                total_balance: 0.0,
                used_balance: 0.0,
                remaining_balance: 0.0,
                currency: "CNY".into(),
                available: false,
                has_api: get_platform_fetcher(&platform.id).is_some(),
                console_url: url.to_string(),
                error: Some("未配置 API Key".into()),
            }
        } else if let Some(fetcher) = get_platform_fetcher(&platform.id) {
            match fetcher.fetch_balance(&platform.api_key).await {
                Ok(info) => info,
                Err(e) => {
                    let (url, name) = get_console_url(&platform.id);
                    BalanceInfo {
                        platform: name.to_string(),
                        platform_id: platform.id.clone(),
                        total_balance: 0.0,
                        used_balance: 0.0,
                        remaining_balance: 0.0,
                        currency: "CNY".into(),
                        available: false,
                        has_api: true,
                        console_url: url.to_string(),
                        error: Some(e.to_string()),
                    }
                }
            }
        } else {
            let (url, name) = get_console_url(&platform.id);
            BalanceInfo {
                platform: name.to_string(),
                platform_id: platform.id.clone(),
                total_balance: 0.0,
                used_balance: 0.0,
                remaining_balance: 0.0,
                currency: "CNY".into(),
                available: false,
                has_api: false,
                console_url: url.to_string(),
                error: None,
            }
        };
        results.push(info);
    }

    results
}

#[tauri::command]
pub fn get_config() -> AppConfig {
    config::load_config()
}

#[tauri::command]
pub fn save_platform_key(platform_id: String, api_key: String) -> Result<(), String> {
    let mut app_config = config::load_config();
    if let Some(p) = app_config.platforms.iter_mut().find(|p| p.id == platform_id) {
        p.api_key = api_key;
    }
    config::save_config(&app_config).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_platform(platform_id: String, enabled: bool) -> Result<(), String> {
    let mut app_config = config::load_config();
    if let Some(p) = app_config.platforms.iter_mut().find(|p| p.id == platform_id) {
        p.enabled = enabled;
    }
    config::save_config(&app_config).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_refresh_interval(seconds: u64) -> Result<(), String> {
    let mut app_config = config::load_config();
    app_config.refresh_interval = seconds;
    config::save_config(&app_config).map_err(|e| e.to_string())
}
