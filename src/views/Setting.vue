<template>
  <div class="setting">
    <div class="setting-body">
      <aside class="setting-sider">
        <div class="setting-sider__header">
          <button class="back-btn" title="返回" aria-label="返回" @click="goBack">
            <n-icon :component="ArrowLeftOutlined" />
          </button>
          <span class="setting-title">设置</span>
        </div>
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
  MessageOutlined,
} from "@vicons/antd"

const router = useRouter()
const route = useRoute()

const menuItems = [
  { label: "外观", name: "Appearance", href: "#appearance", icon: BgColorsOutlined },
  { label: "阅读设置", name: "ReaderSetting", href: "#reader-setting", icon: BgColorsOutlined },
  { label: "快捷键", name: "Shortcut", href: "#shortcut", icon: KeyOutlined },
  { label: "意见反馈", name: "Feedback", href: "#feedback", icon: MessageOutlined },
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
  background: var(--color-window-bg);
}

.setting-sider__header {
  height: 50px;
  display: flex;
  align-items: center;
  padding: 0 12px;
  border-bottom: 1px solid var(--color-border);
}

.back-btn {
  width: 30px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 6px;
  background-color: transparent;
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: color 0.2s ease, background-color 0.2s ease;

  &:hover {
    color: var(--color-accent);
    background-color: var(--color-surface-hover);
  }
}

.setting-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin-left: 10px;
}

.setting-body {
  flex: 1;
  min-height: 0;
  display: flex;
  gap: 12px;
  padding: 12px;
}

.setting-sider {
  flex-shrink: 0;
  width: 176px;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: 8px;
}

.setting-sider__scroll {
  flex: 1;
  min-height: 0;
}

.setting-anchor {
  margin: 8px;

  :deep(.n-anchor-link) {
    margin-bottom: 2px;
  }

  :deep(.n-anchor-link__title) {
    padding: 0;
    color: var(--color-text-secondary);
    border-radius: 6px;
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
  height: 34px;
  padding: 0 10px;
  font-size: 14px;
  border-radius: 6px;

  &__icon {
    margin-right: 8px;
  }

  &--active {
    color: var(--color-accent);
    font-weight: 600;
    background: color-mix(in srgb, var(--color-accent) 10%, transparent);
  }
}

.setting-content {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: 8px;
}

.setting-content__scroll {
  height: 100%;
}

@media (max-width: 640px) {
  .setting-body {
    flex-direction: column;
    gap: 8px;
    padding: 8px;
  }

  .setting-sider {
    width: auto;
    height: 96px;
    overflow: visible;
  }

  .setting-sider__header {
    flex-shrink: 0;
    height: 44px;
    padding: 0 10px;
  }

  .setting-sider__scroll {
    flex: none;
    height: 50px;
  }

  .setting-anchor {
    display: flex;
    align-items: center;
    height: 100%;
    margin: 0 6px;

    :deep(.n-anchor-link) {
      flex: 1;
      margin: 0;
    }
  }

  .nav-item {
    justify-content: center;
    height: 34px;
    padding: 0 6px;

    &__icon {
      margin-right: 5px;
    }
  }
}
</style>
