<template>
  <div class="window-shell" :class="{ 'with-shadow': settingStore.appearance.showShadow }">
    <div class="window-card">
      <header class="header">
        <MacTitleBar v-if="isMacos" />
        <WindowsTitleBar v-else />
      </header>
      <main class="content">
        <RouterView />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import WindowsTitleBar from "@/layout/components/WindowsTitleBar.vue"
import MacTitleBar from "@/layout/components/MacTitleBar.vue"
import { getCurrentWindow } from "@tauri-apps/api/window"
import { useSettingStore } from "@/stores/setting"

const isMacos = navigator.userAgent.includes("Mac OS X")

const appWindow = getCurrentWindow()
const settingStore = useSettingStore()

onMounted(() => {
  // 窗口阴影完全由 CSS box-shadow 呈现（跟随圆角），
  // 因此始终关闭 OS 原生阴影（DWM 阴影只会是直角，无法跟随 CSS 圆角）。
  appWindow.setShadow(false)
})
</script>

<style lang="scss" scoped>
.window-shell {
  width: 100vw;
  height: 100vh;
  padding: var(--window-gap);
  box-sizing: border-box;
  background: transparent;

  &.with-shadow {
    .window-card {
      box-shadow:
        0 6px 18px rgba(0, 0, 0, 0.16),
        0 1px 4px rgba(0, 0, 0, 0.08);
    }
  }
}

.window-card {
  width: 100%;
  height: 100%;
  background: var(--color-window-bg);
  border-radius: var(--radius-window);
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.header {
  flex-shrink: 0;
  padding: 0;
  background: transparent;
}

.content {
  flex: 1;
  min-height: 0;
  background: transparent;
}
</style>
