<template>
  <div class="setting-page">
    <h2 class="page-title">快捷键</h2>
    <p class="page-desc">使用键盘快速完成常用操作</p>

    <div class="setting-group">
      <div v-for="item in shortcuts" :key="item.action" class="shortcut-item">
        <span class="shortcut-action">{{ item.action }}</span>
        <span class="shortcut-keys">
          <template v-for="(key, i) in item.keys" :key="i">
            <kbd class="shortcut-key">{{ key }}</kbd>
            <span v-if="i < item.keys.length - 1" class="shortcut-sep">/</span>
          </template>
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const shortcuts = [
  { action: "上一页", keys: ["←", "PageUp"] },
  { action: "下一页", keys: ["→", "PageDown"] },
  { action: "上一章", keys: ["←"] },
  { action: "下一章", keys: ["→"] },
]
</script>

<style lang="scss" scoped>
.setting-page {
  max-width: 760px;
  padding: 22px 26px 32px;
}

.page-title {
  margin: 0 0 2px;
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.page-desc {
  margin: 0 0 18px;
  font-size: 13px;
  color: var(--color-text-secondary);
}

.setting-group {
  border-top: 1px solid var(--color-border);
  border-bottom: 1px solid var(--color-border);
}

.shortcut-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  min-height: 46px;

  & + & {
    border-top: 1px solid var(--color-border);
  }
}

.shortcut-action {
  font-size: 14px;
  color: var(--color-text-primary);
}

.shortcut-keys {
  display: flex;
  align-items: center;
}

.shortcut-key {
  min-width: 24px;
  height: 22px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  margin: 0 2px;
  border: 1px solid var(--color-border);
  border-radius: 4px;
  background-color: color-mix(in srgb, var(--color-window-bg) 72%, transparent);
  color: var(--color-text-primary);
  font-size: 12px;
  font-family: inherit;
}

.shortcut-sep {
  color: var(--color-text-secondary);
  font-size: 12px;
}

@media (max-width: 640px) {
  .setting-page {
    padding: 18px 16px 24px;
  }
}
</style>
