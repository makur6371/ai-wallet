<script setup lang="ts">
import { ref } from 'vue'
import BalanceList from './components/BalanceList.vue'
import Settings from './components/Settings.vue'

const currentView = ref<'home' | 'settings'>('home')
</script>

<template>
  <div class="app">
    <header class="app-header">
      <div class="logo" @click="currentView = 'home'">
        <span class="logo-icon">W</span>
        <span class="logo-text">AI Wallet</span>
      </div>
      <nav class="nav">
        <button
          class="nav-btn"
          :class="{ active: currentView === 'home' }"
          @click="currentView = 'home'"
        >
          余额
        </button>
        <button
          class="nav-btn"
          :class="{ active: currentView === 'settings' }"
          @click="currentView = 'settings'"
        >
          设置
        </button>
      </nav>
    </header>

    <main class="app-main">
      <BalanceList v-if="currentView === 'home'" />
      <Settings v-else />
    </main>
  </div>
</template>

<style>
@import 'tailwindcss';

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: linear-gradient(135deg, #f5f7fa 0%, #e4ecf7 100%);
  min-height: 100vh;
  color: #1a1a2e;
}

::-webkit-scrollbar {
  width: 6px;
}
::-webkit-scrollbar-track {
  background: transparent;
}
::-webkit-scrollbar-thumb {
  background: #cbd5e1;
  border-radius: 3px;
}
</style>

<style scoped>
.app {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.app-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 24px;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(12px);
  border-bottom: 1px solid rgba(0, 0, 0, 0.06);
  position: sticky;
  top: 0;
  z-index: 10;
}

.logo {
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
  user-select: none;
}
.logo-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  font-weight: 700;
  font-size: 16px;
  border-radius: 10px;
}
.logo-text {
  font-size: 18px;
  font-weight: 700;
  color: #1a1a2e;
}

.nav {
  display: flex;
  gap: 4px;
  background: rgba(0, 0, 0, 0.04);
  padding: 4px;
  border-radius: 12px;
}
.nav-btn {
  padding: 8px 20px;
  border: none;
  border-radius: 10px;
  background: transparent;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  color: #64748b;
  transition: all 0.2s;
}
.nav-btn.active {
  background: white;
  color: #1a1a2e;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
}
.nav-btn:hover:not(.active) {
  color: #1a1a2e;
}

.app-main {
  flex: 1;
  padding: 24px;
  max-width: 960px;
  margin: 0 auto;
  width: 100%;
}
</style>
