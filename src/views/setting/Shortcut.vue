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
            <span v-if="i < item.keys.length - 1" class="shortcut-sep">+</span>
          </template>
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const shortcuts = [
  { action: "上一页", keys: ["←"] },
  { action: "下一页", keys: ["→"] },
  { action: "打开搜索", keys: ["Ctrl", "F"] },
  { action: "打开设置", keys: ["Ctrl", ","] },
]
</script>

<style lang="scss" scoped>
.setting-page {
  margin: 24px 28px;
}

.page-title {
  margin: 0 0 4px;
  font-size: 20px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.page-desc {
  margin: 0 0 20px;
  font-size: 13px;
  color: var(--color-text-secondary);
}

.setting-group {
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  box-shadow: var(--shadow-card);
}

.shortcut-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin: 0 16px;
  min-height: 44px;

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
  height: 24px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  margin: 0 2px;
  border: 1px solid var(--color-border);
  border-radius: 6px;
  background-color: var(--color-window-bg);
  color: var(--color-text-primary);
  font-size: 12px;
  font-family: inherit;
}

.shortcut-sep {
  color: var(--color-text-secondary);
  font-size: 12px;
}
</style>
