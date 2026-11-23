import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { BalanceInfo, AppConfig } from '../types'
import { fetchAllBalances, getConfig, savePlatformKey, togglePlatform } from '../services/api'

export const useBalanceStore = defineStore('balance', () => {
  const balances = ref<BalanceInfo[]>([])
  const config = ref<AppConfig | null>(null)
  const loading = ref(false)
  const lastRefresh = ref<Date | null>(null)
  const error = ref<string | null>(null)

  async function loadConfig() {
    try {
      config.value = await getConfig()
    } catch (e) {
      error.value = String(e)
    }
  }

  async function refresh() {
    loading.value = true
    error.value = null
    try {
      balances.value = await fetchAllBalances()
      lastRefresh.value = new Date()
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  async function updatePlatformKey(platformId: string, apiKey: string) {
    await savePlatformKey(platformId, apiKey)
    await loadConfig()
  }

  async function toggle(platformId: string, enabled: boolean) {
    await togglePlatform(platformId, enabled)
    await loadConfig()
  }

  return {
    balances,
    config,
    loading,
    lastRefresh,
    error,
    loadConfig,
    refresh,
    updatePlatformKey,
    toggle,
  }
})
