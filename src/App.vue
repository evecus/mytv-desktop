<template>
  <div class="app">
    <!-- 顶部栏 -->
    <header class="topbar">
      <div class="brand">📺 我的电视</div>
      <div class="topbar-actions">
        <span v-if="store.speedtestDoneMsg" class="done-badge">✓ {{ store.speedtestDoneMsg }}</span>
        <button class="btn" :disabled="store.speedtestRunning" @click="store.startSpeedtest()">
          <span v-if="store.speedtestRunning" class="spin-inline">⟳</span>
          {{ store.speedtestRunning ? '测速中…' : '测速更新' }}
        </button>
        <button class="btn ghost" @click="store.loadChannels()">刷新</button>
      </div>
    </header>

    <div class="main">
      <!-- 分组列表 -->
      <nav class="sidebar">
        <div v-if="!store.groups.length" class="hint">暂无频道<br>请先测速</div>
        <div
          v-for="g in store.groups"
          :key="g"
          class="group-item"
          :class="{ active: store.activeGroup === g }"
          @click="store.activeGroup = g"
        >{{ g }}</div>
      </nav>

      <!-- 频道列表 -->
      <section class="channel-list">
        <div v-if="!store.filteredChannels.length" class="hint">该分组暂无频道</div>
        <div
          v-for="ch in store.filteredChannels"
          :key="ch.url"
          class="ch-item"
          :class="{ active: store.currentChannel?.url === ch.url }"
          @click="store.playChannel(ch)"
        >
          <img
            v-if="ch.logo"
            :src="ch.logo"
            class="ch-logo"
            @error="e => e.target.style.display='none'"
          />
          <div v-else class="ch-logo-placeholder">TV</div>
          <span class="ch-name">{{ ch.name }}</span>
          <span v-if="store.currentChannel?.url === ch.url" class="playing-indicator">▶</span>
        </div>
      </section>

      <!-- 右侧面板 -->
      <aside class="panel">
        <!-- 正在播放 -->
        <template v-if="store.currentChannel">
          <div class="panel-label">正在播放</div>
          <div class="panel-title">{{ store.currentChannel.name }}</div>
          <div class="panel-sub">{{ store.currentChannel.group }}</div>
          <button class="btn danger mt16 w100" @click="store.stopPlayer()">■ 停止播放</button>
        </template>

        <!-- 测速中 -->
        <template v-else-if="store.speedtestRunning">
          <div class="spinner-wrap">
            <div class="spinner"></div>
          </div>
          <div class="panel-label mt8 center">{{ store.speedtestProgress }}</div>
          <div class="panel-sub center mt4">测速完成后自动刷新</div>
        </template>

        <!-- 空闲 -->
        <template v-else>
          <div class="idle-icon">📺</div>
          <div class="panel-sub center mt8">点击频道开始播放</div>
        </template>

        <div v-if="store.playerError" class="error mt16">⚠ {{ store.playerError }}</div>
        <div v-if="store.speedtestError" class="error mt16">⚠ {{ store.speedtestError }}</div>
      </aside>
    </div>

    <!-- 测速遮罩 -->
    <Transition name="fade">
      <div v-if="store.speedtestRunning" class="overlay">
        <div class="overlay-card">
          <div class="spinner large"></div>
          <div class="overlay-title">正在测速</div>
          <div class="overlay-progress">{{ store.speedtestProgress }}</div>
          <div class="overlay-hint">测速过程约 2～5 分钟，完成后自动更新频道列表</div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup>
import { onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useTVStore } from './stores/tv.js'

const store = useTVStore()

onMounted(async () => {
  store.setupListeners()
  const has = await invoke('has_local_source')
  if (has) await store.loadChannels()
})
</script>

<style>
*, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }

:root {
  --bg:      #161b2a;
  --bg2:     #1e253a;
  --bg3:     #252d45;
  --accent:  #1976D2;
  --accent2: #42a5f5;
  --text:    #e8eaf6;
  --text2:   #8c9abb;
  --border:  #2a3454;
  --green:   #00c853;
  --red:     #ef5350;
  --r:       8px;
}

body {
  font-family: 'PingFang SC', 'Microsoft YaHei', system-ui, sans-serif;
  background: var(--bg);
  color: var(--text);
  height: 100vh;
  overflow: hidden;
  user-select: none;
}

.app { display: flex; flex-direction: column; height: 100vh; }

/* Topbar */
.topbar {
  display: flex; align-items: center; justify-content: space-between;
  height: 52px; padding: 0 20px;
  background: var(--bg2); border-bottom: 1px solid var(--border);
  flex-shrink: 0;
}
.brand { font-size: 17px; font-weight: 700; letter-spacing: .5px; }
.topbar-actions { display: flex; align-items: center; gap: 10px; }
.done-badge {
  font-size: 12px; color: var(--green);
  background: rgba(0,200,83,.1); border: 1px solid rgba(0,200,83,.3);
  padding: 3px 10px; border-radius: 20px;
}

/* Buttons */
.btn {
  padding: 6px 18px; border-radius: var(--r); border: none;
  background: var(--accent); color: #fff; font-size: 13px;
  cursor: pointer; transition: background .15s; display: flex; align-items: center; gap: 6px;
}
.btn:hover:not(:disabled) { background: var(--accent2); }
.btn:disabled { opacity: .45; cursor: default; }
.btn.ghost { background: var(--bg3); color: var(--text2); }
.btn.ghost:hover { background: var(--border); color: var(--text); }
.btn.danger { background: #b71c1c; }
.btn.danger:hover { background: var(--red); }
.mt16 { margin-top: 16px; }
.w100 { width: 100%; justify-content: center; }

/* Layout */
.main { display: flex; flex: 1; overflow: hidden; }

/* Sidebar */
.sidebar {
  width: 130px; background: var(--bg2); border-right: 1px solid var(--border);
  overflow-y: auto; flex-shrink: 0;
}
.group-item {
  padding: 12px 16px; font-size: 13px; color: var(--text2);
  cursor: pointer; border-left: 3px solid transparent; transition: all .15s;
}
.group-item:hover { background: var(--bg3); color: var(--text); }
.group-item.active { background: var(--bg3); color: var(--text); border-left-color: var(--accent2); }

/* Channel list */
.channel-list { flex: 1; overflow-y: auto; padding: 6px 8px; display: flex; flex-direction: column; gap: 3px; }
.ch-item {
  display: flex; align-items: center; gap: 10px;
  padding: 9px 12px; border-radius: var(--r);
  cursor: pointer; transition: background .12s; border: 1px solid transparent;
}
.ch-item:hover { background: var(--bg3); }
.ch-item.active { background: var(--bg3); border-color: var(--accent); }
.ch-logo { width: 30px; height: 30px; object-fit: contain; border-radius: 4px; flex-shrink: 0; }
.ch-logo-placeholder {
  width: 30px; height: 30px; border-radius: 4px; flex-shrink: 0;
  background: var(--bg3); display: flex; align-items: center; justify-content: center;
  font-size: 10px; color: var(--text2); font-weight: 700;
}
.ch-name { flex: 1; font-size: 14px; }
.playing-indicator { color: var(--green); font-size: 11px; }

/* Right panel */
.panel {
  width: 190px; background: var(--bg2); border-left: 1px solid var(--border);
  padding: 20px 16px; flex-shrink: 0; display: flex; flex-direction: column;
}
.panel-label { font-size: 11px; color: var(--text2); letter-spacing: 1px; text-transform: uppercase; }
.panel-title { font-size: 19px; font-weight: 700; margin-top: 8px; line-height: 1.3; }
.panel-sub { font-size: 12px; color: var(--text2); margin-top: 4px; }
.idle-icon { font-size: 36px; text-align: center; margin-top: 32px; opacity: .4; }
.center { text-align: center; }
.mt8 { margin-top: 8px; }
.mt4 { margin-top: 4px; }
.error { font-size: 12px; color: var(--red); line-height: 1.6; }
.spinner-wrap { display: flex; justify-content: center; margin-top: 28px; }

/* Spinner */
.spinner {
  width: 32px; height: 32px;
  border: 3px solid var(--border); border-top-color: var(--accent2);
  border-radius: 50%; animation: spin .8s linear infinite;
}
.spinner.large { width: 52px; height: 52px; border-width: 4px; }
@keyframes spin { to { transform: rotate(360deg); } }
.spin-inline { display: inline-block; animation: spin .8s linear infinite; }

/* Hint */
.hint { padding: 20px 12px; font-size: 12px; color: var(--text2); text-align: center; line-height: 1.8; }

/* Overlay */
.overlay {
  position: fixed; inset: 0; z-index: 99;
  background: rgba(10, 14, 28, .8); backdrop-filter: blur(6px);
  display: flex; align-items: center; justify-content: center;
}
.overlay-card {
  background: var(--bg2); border: 1px solid var(--border);
  border-radius: 16px; padding: 48px 56px; text-align: center; min-width: 320px;
}
.overlay-title  { font-size: 22px; font-weight: 700; margin: 24px 0 10px; }
.overlay-progress { font-size: 14px; color: var(--text2); min-height: 22px; }
.overlay-hint { font-size: 12px; color: var(--text2); margin-top: 18px; opacity: .6; }

/* Scrollbar */
::-webkit-scrollbar { width: 4px; }
::-webkit-scrollbar-track { background: transparent; }
::-webkit-scrollbar-thumb { background: var(--border); border-radius: 2px; }
::-webkit-scrollbar-thumb:hover { background: var(--text2); }

/* Transition */
.fade-enter-active, .fade-leave-active { transition: opacity .25s; }
.fade-enter-from, .fade-leave-to { opacity: 0; }
</style>
