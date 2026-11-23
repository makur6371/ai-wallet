<script setup lang="ts">
import type { BalanceInfo } from '../types'

defineProps<{
  info: BalanceInfo
}>()

function formatBalance(val: number): string {
  if (val >= 10000) return (val / 10000).toFixed(2) + '万'
  return val.toFixed(2)
}
</script>

<template>
  <div class="card" :class="{ disabled: !info.available && info.has_api, 'no-api': !info.has_api }">
    <div class="card-header">
      <span class="platform-name">{{ info.platform }}</span>
      <span class="status-dot" :class="{ online: info.available, offline: !info.available }"></span>
    </div>

    <div v-if="info.error" class="error-msg">
      {{ info.error }}
      <a v-if="info.console_url && !info.has_api" :href="info.console_url" target="_blank" class="console-link">
        前往控制台 &rarr;
      </a>
    </div>

    <div v-else-if="info.has_api" class="balance-grid">
      <div class="balance-item">
        <span class="label">余额</span>
        <span class="value">¥{{ formatBalance(info.remaining_balance) }}</span>
      </div>
      <div class="balance-item">
        <span class="label">总额</span>
        <span class="value sub">¥{{ formatBalance(info.total_balance) }}</span>
      </div>
      <div class="balance-item">
        <span class="label">已用</span>
        <span class="value sub">¥{{ formatBalance(info.used_balance) }}</span>
      </div>
    </div>

    <div v-else class="no-api-msg">
      <span>暂无余额查询 API</span>
      <a v-if="info.console_url" :href="info.console_url" target="_blank" class="console-link">
        前往控制台 &rarr;
      </a>
    </div>
  </div>
</template>

<style scoped>
.card {
  background: rgba(255, 255, 255, 0.7);
  backdrop-filter: blur(12px);
  border: 1px solid rgba(0, 0, 0, 0.06);
  border-radius: 16px;
  padding: 20px;
  transition: all 0.2s ease;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
}
.card:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.08);
}
.card.disabled {
  opacity: 0.6;
}
.card.no-api {
  background: rgba(255, 255, 255, 0.4);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}
.platform-name {
  font-size: 16px;
  font-weight: 600;
  color: #1a1a2e;
}
.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}
.status-dot.online {
  background: #22c55e;
  box-shadow: 0 0 6px rgba(34, 197, 94, 0.5);
}
.status-dot.offline {
  background: #94a3b8;
}

.balance-grid {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 12px;
}
.balance-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.label {
  font-size: 12px;
  color: #94a3b8;
}
.value {
  font-size: 20px;
  font-weight: 700;
  color: #1a1a2e;
}
.value.sub {
  font-size: 14px;
  font-weight: 500;
  color: #64748b;
}

.error-msg {
  font-size: 13px;
  color: #ef4444;
  padding: 8px 0;
}
.no-api-msg {
  font-size: 13px;
  color: #94a3b8;
  padding: 8px 0;
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.console-link {
  font-size: 13px;
  color: #3b82f6;
  text-decoration: none;
}
.console-link:hover {
  text-decoration: underline;
}
</style>
