<template>
  <div class="setting-page">
    <h2 class="page-title">外观</h2>
    <p class="page-desc">自定义应用的外观与阅读体验</p>

    <div class="setting-group">
      <div class="group-title">主题</div>
      <div class="setting-item">
        <div class="item-info">
          <span class="item-label">主题模式</span>
          <span class="item-desc">选择应用的整体配色方案</span>
        </div>
        <n-radio-group v-model:value="theme" name="theme">
          <n-radio-button value="light">浅色</n-radio-button>
          <n-radio-button value="dark">深色</n-radio-button>
          <n-radio-button value="system">跟随系统</n-radio-button>
        </n-radio-group>
      </div>
    </div>
    

    <div class="setting-group">
      <div class="group-title">阅读</div>
      <div class="setting-item">
        <div class="item-info">
          <span class="item-label">自动换行</span>
          <span class="item-desc">长段落自动换行显示</span>
        </div>
        <n-switch v-model:value="wordWrap" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const theme = ref("light")
const wordWrap = ref(true)
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
  margin-bottom: 24px;
}

.group-title {
  margin-bottom: 8px;
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-secondary);
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  box-shadow: var(--shadow-card);
  margin-bottom: 0;
  transition: box-shadow 0.2s ease, border-color 0.2s ease;

  &:hover {
    border-color: var(--color-border-hover);
    box-shadow: var(--shadow-card-hover);
  }
}

.item-info {
  display: flex;
  flex-direction: column;
  margin: 12px 16px;
}

.item-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text-primary);
}

.item-desc {
  font-size: 12px;
  color: var(--color-text-secondary);
  margin-top: 2px;
}

:deep(.n-radio-group),
:deep(.n-switch) {
  margin: 0 16px;
}
</style>
