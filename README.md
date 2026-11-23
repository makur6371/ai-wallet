<div align="center">

[![GitHub stars](https://img.shields.io/github/stars/makur6371/ai-wallet?style=social)](https://github.com/makur6371/ai-wallet)
[![GitHub forks](https://img.shields.io/github/forks/makur6371/ai-wallet?style=social)](https://github.com/makur6371/ai-wallet)
[![Tauri](https://img.shields.io/badge/Tauri-FFC131?logo=tauri&logoColor=black)](https://tauri.app)
[![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Vue](https://img.shields.io/badge/Vue-42B883?logo=vuedotjs&logoColor=white)](https://vuejs.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=white)](https://www.typescriptlang.org)

</div>

# AI Wallet

多平台 AI API 余额监控桌面应用 —— 在一个界面里查看所有 AI 服务的余额与剩余额度。

支持 DeepSeek、智谱 GLM、Kimi/Moonshot、MiniMax、百川、百度千帆、通义千问等平台。

## 功能

- **余额总览** — 总余额 / 已用 / 剩余，跨平台汇总
- **平台卡片** — 每个平台独立展示，支持单独启停
- **密钥管理** — API Key 本地存储，界面内掩码显示
- **控制台直达** — 一键跳转各平台控制台

## 技术栈

- **前端**：Vue 3 + TypeScript + Vite + Pinia
- **后端**：Rust + [Tauri 2](https://tauri.app/)
- **UI 组件**：`BalanceList.vue`、`PlatformCard.vue`、`Settings.vue`

## 架构

```
src/                  # Vue 前端
  components/         # BalanceList / PlatformCard / Settings
  stores/balance.ts   # Pinia 状态
  services/api.ts     # Tauri command 调用封装
src-tauri/            # Rust 后端
  src/commands.rs     # fetch_all_balances / save_platform_key
  src/config.rs       # 配置读写
  src/models.rs       # AppConfig / PlatformConfig / BalanceInfo
  src/platforms/      # 各平台余额查询实现
```

## 密钥存储

API Key **不写入仓库**，运行时保存在系统配置目录：

```
%APPDATA%\ai-wallet\config.json
```

## 本地运行

```bash
npm install
npm run tauri dev
```

### 打包

```bash
npm run tauri build
```

## 说明

本项目为个人自用项目，代码仅供学习参考。
