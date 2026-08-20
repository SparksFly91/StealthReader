import { defineStore } from "pinia"

/**
 * 全局加载状态 store
 * 支持嵌套调用（count 计数），loading 为 true 表示当前有任务在进行
 */
export const useLoadingStore = defineStore("loading", {
  state: () => ({
    count: 0,
    text: "加载中...",
  }),
  getters: {
    loading: (state) => state.count > 0,
  },
  actions: {
    start(text?: string) {
      if (text) this.text = text
      this.count++
    },
    stop() {
      this.count = Math.max(0, this.count - 1)
    },
    reset() {
      this.count = 0
      this.text = "加载中..."
    },
  },
})
