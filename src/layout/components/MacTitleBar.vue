<template>
  <div class="titlebar" data-tauri-drag-region>
    <div class="traffic-lights">
      <button class="light close" title="关闭" @click="close">
        <n-icon :size="8"><CloseOutline /></n-icon>
      </button>
      <button class="light minimize" title="最小化" @click="minimize">
        <n-icon :size="8"><RemoveOutline /></n-icon>
      </button>
    </div>
    <div class="title">幽灵阅读器</div>
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
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  height: var(--titlebar-height);
  user-select: none;
  -webkit-user-select: none;
  background: color-mix(in srgb, var(--color-window-bg) 60%, transparent);
  border-top-left-radius: var(--radius-window);
  border-top-right-radius: var(--radius-window);

  .traffic-lights {
    position: absolute;
    left: 14px;
    top: 50%;
    transform: translateY(-50%);
    display: flex;
    align-items: center;
    gap: 8px;

    .light {
      width: 12px;
      height: 12px;
      padding: 0;
      border-radius: var(--radius-sm);
      border: 0.5px solid rgba(0, 0, 0, 0.15);
      display: flex;
      align-items: center;
      justify-content: center;
      cursor: default;

      :deep(.n-icon) {
        opacity: 0;
        transition: opacity 0.15s ease;
      }

      &:hover :deep(.n-icon) {
        opacity: 1;
      }

      &.close {
        background: #ff5f57;
        color: #820d0d;
      }

      &.minimize {
        background: #febc2e;
        color: #995700;
      }
    }
  }

  .title {
    font-size: 13px;
    color: var(--color-text-secondary);
  }
}
</style>