<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useBalanceStore } from '../stores/balance'
import { setRefreshInterval } from '../services/api'

const store = useBalanceStore()
const editingId = ref<string | null>(null)
const keyValue = ref('')
const refreshValue = ref(300)

onMounted(async () => {
  await store.loadConfig()
  refreshValue.value = store.config?.refresh_interval ?? 300
})

const platforms = computed(() => store.config?.platforms ?? [])

function startEdit(id: string, currentKey: string) {
  editingId.value = id
  keyValue.value = currentKey
}

async function saveKey() {
  if (!editingId.value) return
  await store.updatePlatformKey(editingId.value, keyValue.value)
  editingId.value = null
  keyValue.value = ''
}

function cancelEdit() {
  editingId.value = null
  keyValue.value = ''
}

async function saveRefresh() {
  await setRefreshInterval(refreshValue.value)
  await store.loadConfig()
}

function maskKey(key: string): string {
  if (!key) return '未设置'
  if (key.length <= 8) return '****'
  return key.slice(0, 4) + '****' + key.slice(-4)
}
</script>

<template>
  <div class="settings">
    <h2>设置</h2>

    <div class="section">
      <h3>API Key 管理</h3>
      <p class="hint">配置各平台的 API Key 以查询余额</p>

      <div class="platform-list">
        <div v-for="p in platforms" :key="p.id" class="platform-row">
          <div class="platform-info">
            <span class="name">{{ p.name }}</span>
            <span class="key-preview">{{ maskKey(p.api_key) }}</span>
          </div>

          <div class="platform-actions">
            <label class="toggle">
              <input type="checkbox" :checked="p.enabled" @change="store.toggle(p.id, ($event.target as HTMLInputElement).checked)" />
              <span class="slider"></span>
            </label>
            <button class="edit-btn" @click="startEdit(p.id, p.api_key)">
              {{ editingId === p.id ? '编辑中' : '编辑' }}
            </button>
          </div>

          <div v-if="editingId === p.id" class="edit-row">
            <input
              v-model="keyValue"
              type="password"
              placeholder="输入 API Key..."
              class="key-input"
              @keyup.enter="saveKey"
              @keyup.escape="cancelEdit"
            />
            <button class="save-btn" @click="saveKey">保存</button>
            <button class="cancel-btn" @click="cancelEdit">取消</button>
          </div>
        </div>
      </div>
    </div>

    <div class="section">
      <h3>刷新间隔</h3>
      <div class="interval-row">
        <input v-model.number="refreshValue" type="number" min="30" max="3600" class="interval-input" />
        <span class="unit">秒</span>
        <button class="save-btn" @click="saveRefresh">保存</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings {
  display: flex;
  flex-direction: column;
  gap: 24px;
}
h2 {
  font-size: 20px;
  font-weight: 700;
  color: #1a1a2e;
  margin: 0;
}
h3 {
  font-size: 16px;
  font-weight: 600;
  color: #1a1a2e;
  margin: 0 0 4px 0;
}
.hint {
  font-size: 13px;
  color: #94a3b8;
  margin: 0 0 12px 0;
}

.section {
  background: rgba(255, 255, 255, 0.7);
  backdrop-filter: blur(12px);
  border: 1px solid rgba(0, 0, 0, 0.06);
  border-radius: 16px;
  padding: 20px;
}

.platform-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.platform-row {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 12px;
  padding: 12px 0;
  border-bottom: 1px solid rgba(0, 0, 0, 0.04);
}
.platform-row:last-child {
  border-bottom: none;
}

.platform-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}
.name {
  font-weight: 500;
  font-size: 14px;
  color: #1a1a2e;
}
.key-preview {
  font-size: 12px;
  color: #94a3b8;
  font-family: monospace;
}

.platform-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.edit-btn {
  padding: 6px 14px;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  background: white;
  font-size: 13px;
  cursor: pointer;
  color: #64748b;
}
.edit-btn:hover {
  border-color: #3b82f6;
  color: #3b82f6;
}

.edit-row {
  width: 100%;
  display: flex;
  gap: 8px;
  margin-top: 8px;
}
.key-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  font-size: 14px;
  outline: none;
}
.key-input:focus {
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.save-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 8px;
  background: #3b82f6;
  color: white;
  font-size: 13px;
  cursor: pointer;
}
.save-btn:hover {
  background: #2563eb;
}
.cancel-btn {
  padding: 8px 16px;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  background: white;
  font-size: 13px;
  cursor: pointer;
  color: #64748b;
}

/* Toggle switch */
.toggle {
  position: relative;
  display: inline-block;
  width: 40px;
  height: 22px;
}
.toggle input {
  opacity: 0;
  width: 0;
  height: 0;
}
.slider {
  position: absolute;
  cursor: pointer;
  inset: 0;
  background: #cbd5e1;
  border-radius: 22px;
  transition: 0.2s;
}
.slider::before {
  content: '';
  position: absolute;
  height: 16px;
  width: 16px;
  left: 3px;
  bottom: 3px;
  background: white;
  border-radius: 50%;
  transition: 0.2s;
}
.toggle input:checked + .slider {
  background: #3b82f6;
}
.toggle input:checked + .slider::before {
  transform: translateX(18px);
}

.interval-row {
  display: flex;
  align-items: center;
  gap: 8px;
}
.interval-input {
  width: 80px;
  padding: 8px 12px;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  font-size: 14px;
  outline: none;
}
.interval-input:focus {
  border-color: #3b82f6;
}
.unit {
  font-size: 14px;
  color: #64748b;
}
</style>
