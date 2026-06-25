import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export const useTVStore = defineStore('tv', {
  state: () => ({
    channels: [],
    groups: [],
    activeGroup: '',
    currentChannel: null,
    isPlaying: false,
    playerError: null,
    speedtestRunning: false,
    speedtestProgress: '',
    speedtestError: null,
    speedtestDoneMsg: '',
  }),

  getters: {
    filteredChannels(state) {
      if (!state.activeGroup) return state.channels
      return state.channels.filter(c => c.group === state.activeGroup)
    },
  },

  actions: {
    async loadChannels() {
      try {
        const list = await invoke('load_channels')
        this.channels = list
        const seen = new Set()
        this.groups = []
        for (const ch of list) {
          if (!seen.has(ch.group)) { seen.add(ch.group); this.groups.push(ch.group) }
        }
        if (this.groups.length && !this.activeGroup) {
          this.activeGroup = this.groups[0]
        }
      } catch (e) {
        console.error('load_channels', e)
      }
    },

    async playChannel(channel) {
      this.currentChannel = channel
      this.playerError = null
      this.isPlaying = true
      try {
        await invoke('play_url', { url: channel.url, title: channel.name })
      } catch (e) {
        this.playerError = String(e)
        this.isPlaying = false
      }
    },

    async stopPlayer() {
      await invoke('stop_player')
      this.isPlaying = false
      this.currentChannel = null
    },

    async startSpeedtest() {
      this.speedtestRunning = true
      this.speedtestProgress = '准备中…'
      this.speedtestError = null
      this.speedtestDoneMsg = ''
      try {
        await invoke('start_speedtest', { workers: 60, top: 10 })
      } catch (e) {
        this.speedtestRunning = false
        this.speedtestError = String(e)
      }
    },

    setupListeners() {
      listen('speedtest://progress', e => { this.speedtestProgress = e.payload })
      listen('speedtest://done', e => {
        this.speedtestRunning = false
        this.speedtestDoneMsg = `测速完成，共 ${e.payload} 个频道`
        this.loadChannels()
      })
      listen('speedtest://error', e => {
        this.speedtestRunning = false
        this.speedtestError = e.payload
      })
      listen('player://error', e => {
        this.playerError = e.payload
        this.isPlaying = false
      })
    },
  },
})
