<template>  <div class="home">
    <header class="home-header">
      <n-input
        v-model:value="searchValue"
        class="search-input"
        round
        clearable
        placeholder="搜索书籍"
      >
        <template #prefix>
          <n-icon :component="BookSearch24Regular" />
        </template>
      </n-input>

      <button class="setting-btn" title="设置" aria-label="设置" @click="goSettingView">
        <n-icon :size="20" :component="SettingOutlined" />
      </button>
    </header>

    <section class="home-body">
      <div class="section-title">
        <span class="section-name">我的书籍</span>
        <span class="section-count">{{ bookList.length }} 本</span>
      </div>

      <n-scrollbar class="book-scrollbar">
        <n-empty v-if="bookList.length === 0" class="empty" description="还没有导入书籍哦">
          <template #extra>
            <n-button size="small" type="primary" round>立即导入</n-button>
          </template>
        </n-empty>
      </n-scrollbar>
    </section>
  </div>
</template>

<script setup lang="ts">
import { BookSearch24Regular } from "@vicons/fluent"
import { SettingOutlined } from "@vicons/antd"

const router = useRouter()

const bookList = ref([])
const searchValue = ref("")

const goSettingView = () => {
  router.push({ name: "Setting" })
}
</script>

<style lang="scss" scoped>
.home {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.home-header {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  margin: 20px 24px 12px;
}

.search-input {
  flex: 1;
  margin-right: 16px;

  :deep(.n-input) {
    background-color: var(--color-surface);
    border-color: var(--color-border);
  }

  :deep(.n-input:hover) {
    border-color: var(--color-border-hover);
  }

  :deep(.n-input.n-input--focus) {
    border-color: var(--color-accent);
    box-shadow: var(--shadow-focus);
  }
}

.setting-btn {
  flex-shrink: 0;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: var(--radius-sm);
  background-color: var(--color-surface);
  color: var(--color-text-secondary);
  box-shadow: var(--shadow-card);
  cursor: pointer;
  transition: color 0.2s ease, box-shadow 0.2s ease, background-color 0.2s ease;

  &:hover {
    color: var(--color-accent);
    background-color: var(--color-surface-hover);
    box-shadow: var(--shadow-card-hover);
  }
}

.home-body {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  margin: 0 24px 20px;
}

.section-title {
  flex-shrink: 0;
  display: flex;
  align-items: baseline;
  margin-bottom: 12px;
}

.section-name {
  font-size: 15px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin-right: 8px;
}

.section-count {
  font-size: 12px;
  color: var(--color-text-secondary);
}

.book-scrollbar {
  flex: 1;
  min-height: 0;

  :deep(.n-scrollbar-container) {
    height: 100%;
  }

  :deep(.n-scrollbar-content) {
    min-height: 100%;
  }
}

.empty {
  min-height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}
</style>
pt>

<style lang="scss" scoped></style>
