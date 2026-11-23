import { invoke } from '@tauri-apps/api/core'
import type { BalanceInfo, AppConfig } from '../types'

export async function fetchAllBalances(): Promise<BalanceInfo[]> {
  return invoke('fetch_all_balances')
}

export async function getConfig(): Promise<AppConfig> {
  return invoke('get_config')
}

export async function savePlatformKey(platformId: string, apiKey: string): Promise<void> {
  return invoke('save_platform_key', { platformId, apiKey })
}

export async function togglePlatform(platformId: string, enabled: boolean): Promise<void> {
  return invoke('toggle_platform', { platformId, enabled })
}

export async function setRefreshInterval(seconds: number): Promise<void> {
  return invoke('set_refresh_interval', { seconds })
}
