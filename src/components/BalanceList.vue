<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useBalanceStore } from '../stores/balance'
import PlatformCard from './PlatformCard.vue'

const store = useBalanceStore()
const autoRefreshTimer = ref<ReturnType<typeof setInterval> | null>(null)

onMounted(async () => {
  await store.loadConfig()
  await store.refresh()
  startAutoRefresh()
})

function startAutoRefresh() {
  stopAutoRefresh()
  const interval = (store.config?.refresh_interval ?? 300) * 1000
  autoRefreshTimer.value = setInterval(() => store.refresh(), interval)
}

function stopAutoRefresh() {
  if (autoRefreshTimer.value) {
    clearInterval(autoRefreshTimer.value)
    autoRefreshTimer.value = null
  }
}

function formatTime(date: Date | null): string {
  if (!date) return '--'
  return date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit', second: '2-digit' })
}
</script>

<template>
  <div class="balance-list">
    <div class="toolbar">
      <div class="refresh-info">
        <span v-if="store.lastRefresh">上次刷新: {{ formatTime(store.lastRefresh) }}</span>
      </div>
      <button class="refresh-btn" :disabled="store.loading" @click="store.refresh()">
        <span v-if="store.loading" class="spinner"></span>
        <span v-else>刷新</span>
      </button>
    </div>

    <div v-if="store.error" class="global-error">{{ store.error }}</div>

    <div class="cards-grid">
      <PlatformCard
        v-for="item in store.balances"
        :key="item.platform_id"
        :info="item"
      />
    </div>

    <div v-if="!store.loading && store.balances.length === 0" class="empty">
      暂无数据，请在设置中添加 API Key
    </div>
  </div>
</template>

<style scoped>
.balance-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.refresh-info {
  font-size: 13px;
  color: #94a3b8;
}

.refresh-btn {
  padding: 8px 20px;
  border: none;
  border-radius: 10px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}
.refresh-btn:hover:not(:disabled) {
  transform: scale(1.02);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}
.refresh-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.spinner {
  display: inline-block;
  width: 14px;
  height: 14px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-top-color: white;
  border-radius: 50%;
  animation: spin 0.6s linear infinite;
}
@keyframes spin {
  to { transform: rotate(360deg); }
}

.cards-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
  gap: 16px;
}

.global-error {
  background: #fef2f2;
  color: #ef4444;
  padding: 12px 16px;
  border-radius: 10px;
  font-size: 14px;
}

.empty {
  text-align: center;
  padding: 60px 20px;
  color: #94a3b8;
  font-size: 15px;
}
</style>
