export interface BalanceInfo {
  platform: string
  platform_id: string
  total_balance: number
  used_balance: number
  remaining_balance: number
  currency: string
  available: boolean
  has_api: boolean
  console_url: string
  error: string | null
}

export interface PlatformConfig {
  id: string
  name: string
  api_key: string
  enabled: boolean
}

export interface AppConfig {
  platforms: PlatformConfig[]
  refresh_interval: number
}
