<template>
  <div class="appearance">
    <h2 class="page-title">外观</h2>
    <p class="page-desc">自定义应用的外观与阅读体验</p>

    <n-grid :cols="4">
      <n-gi :span="2">
        <div class="set-item-title">主题模式</div>
        <div class="set-item-desc">选择应用的整体配色方案</div>
      </n-gi>
      <n-gi>
        <n-radio-group v-model:value="theme" size="small" name="theme">
          <n-radio-button value="light">浅色</n-radio-button>
          <n-radio-button value="dark">深色</n-radio-button>
          <n-radio-button value="system">跟随系统</n-radio-button>
        </n-radio-group>
      </n-gi>
      <n-gi :span="4">
        <n-divider style="margin: 16px 0" />
      </n-gi>
      <n-gi :span="3">
        <div class="set-item-title">窗口阴影</div>
        <div class="set-item-desc">是否显示应用窗口阴影效果</div>
      </n-gi>
      <n-gi>
        <n-switch v-model:value="showShadow" />
      </n-gi>
    </n-grid>
  </div>
</template>

<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useSettingStore } from '@/stores/setting'

const settingStore = useSettingStore()

const theme = ref(settingStore.appearance.theme)
const showShadow = ref(settingStore.appearance.showShadow)

watch(showShadow, (newVal) => {
  settingStore.appearance.showShadow = newVal
  getCurrentWindow().setShadow(newVal)
})
</script>

<style lang="scss" scoped>
.appearance {
  padding: 20px;
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

.set-item-title {
  font-size: var(--set-item-title-size);
  font-weight: bold;
}

.set-item-desc {
  font-size: var(--set-item-desc-size);
  color: #666;
}

:deep(.n-radio-group) {
  flex-wrap: wrap;
  justify-content: flex-end;
}
</style>
