<template>
  <div class="setting">
    <header class="setting-header">
      <button class="back-btn" title="返回" aria-label="返回" @click="goBack">
        <n-icon :component="ArrowLeftOutlined" />
      </button>
      <span class="setting-title">设置</span>
    </header>

    <div class="setting-body">
      <aside class="setting-sider">
        <n-scrollbar class="setting-sider__scroll">
          <n-anchor
            class="setting-anchor"
            :show-rail="false"
            :show-background="false"
          >
            <n-anchor-link
              v-for="item in menuItems"
              :key="item.name"
              :href="item.href"
              @click="onNavClick(item, $event)"
            >
              <template #title>
                <span class="nav-item" :class="{ 'nav-item--active': isActive(item) }">
                  <n-icon class="nav-item__icon" :component="item.icon" :size="18" />
                  <span class="nav-item__label">{{ item.label }}</span>
                </span>
              </template>
            </n-anchor-link>
          </n-anchor>
        </n-scrollbar>
      </aside>

      <section class="setting-content">
        <n-scrollbar class="setting-content__scroll">
          <RouterView />
        </n-scrollbar>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  ArrowLeftOutlined,
  BgColorsOutlined,
  InfoCircleOutlined,
  KeyOutlined,
} from "@vicons/antd"

const router = useRouter()
const route = useRoute()

const menuItems = [
  { label: "外观", name: "Appearance", href: "#appearance", icon: BgColorsOutlined },
  { label: "快捷键", name: "Shortcut", href: "#shortcut", icon: KeyOutlined },
  { label: "关于", name: "About", href: "#about", icon: InfoCircleOutlined },
]

const goBack = () => {
  router.push({ name: "Home" })
}

const isActive = (item: (typeof menuItems)[number]) => route.name === item.name

const onNavClick = (item: (typeof menuItems)[number], e: Event) => {
  e.preventDefault()
  router.push({ name: item.name })
}
</script>

<style lang="scss" scoped>
.setting {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.setting-header {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  margin: 20px 24px 16px;
}

.back-btn {
  width: 36px;
  height: 36px;
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

.setting-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin-left: 12px;
}

.setting-body {
  flex: 1;
  min-height: 0;
  display: flex;
  margin: 0 24px 24px;
}

.setting-sider {
  flex-shrink: 0;
  width: 200px;
  margin-right: 24px;
  overflow: hidden;
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-card);
  box-shadow: var(--shadow-card);
}

.setting-sider__scroll {
  height: 100%;
}

.setting-anchor {
  margin: 12px;

  :deep(.n-anchor-link) {
    margin-bottom: 4px;
  }

  :deep(.n-anchor-link__title) {
    padding: 0;
    color: var(--color-text-secondary);
    border-radius: var(--radius-sm);
    transition: color 0.2s ease, background-color 0.2s ease;
  }

  :deep(.n-anchor-link__title:hover) {
    color: var(--color-text-primary);
    background-color: var(--color-surface-hover);
  }
}

.nav-item {
  display: flex;
  align-items: center;
  margin: 8px 12px;
  font-size: 14px;

  &__icon {
    margin-right: 10px;
  }

  &--active {
    color: var(--color-accent);
    font-weight: 600;
  }
}

.setting-content {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-card);
  box-shadow: var(--shadow-card);
}

.setting-content__scroll {
  height: 100%;
}
</style>
