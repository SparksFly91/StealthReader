<template>
  <div class="titlebar" data-tauri-drag-region>
    <div class="brand">
      <!-- <span class="dot" /> -->
      <span class="name">幽灵阅读器</span>
    </div>
    <div class="window-controls">
      <button class="control" title="最小化" @click="minimize">
        <n-icon :size="14"><RemoveOutline /></n-icon>
      </button>
      <button class="control close" title="关闭" @click="close">
        <n-icon :size="14"><CloseOutline /></n-icon>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window"
import { CloseOutline, RemoveOutline } from "@vicons/ionicons5"

const appWindow = getCurrentWindow()

function minimize() {
  appWindow.minimize()
}

function close() {
  appWindow.close()
}
</script>

<style scoped lang="scss">
.titlebar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: var(--titlebar-height);
  padding-left: 12px;
  user-select: none;
  -webkit-user-select: none;
  background: color-mix(in srgb, var(--color-window-bg) 60%, transparent);

  .brand {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 13px;
    color: var(--color-text-secondary);

    .dot {
      width: 8px;
      height: 8px;
      border-radius: 50%;
      background: var(--color-accent-gradient);
    }
  }

  .window-controls {
    display: flex;
    height: 100%;
    overflow: hidden;

    .control {
      width: 46px;
      height: 100%;
      padding: 0;
      border: none;
      background: transparent;
      display: flex;
      align-items: center;
      justify-content: center;
      color: var(--color-text-primary);
      cursor: default;
      transition: background-color 0.15s ease;

      &:hover {
        background: rgba(0, 0, 0, 0.08);
      }

      &.close:hover {
        background: #e81123;
        color: #fff;
      }
    }
  }
}
</style>
