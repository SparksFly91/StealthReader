<template>
  <n-space vertical size="large" class="container">
    <n-layout>
      <n-layout-header class="header">
        <MacTitleBar v-if="isMacos" />
        <WindowsTitleBar v-else />
      </n-layout-header>
      <n-layout-content class="content"> 
        <RouterView />
      </n-layout-content>
    </n-layout>
  </n-space>
</template>

<script setup lang="ts">
import WindowsTitleBar from "@/layout/components/WindowsTitleBar.vue"
import MacTitleBar from "@/layout/components/MacTitleBar.vue"

const isMacos = navigator.userAgent.includes("Mac OS X")
</script>

<style lang="scss" scoped>
.container {
  width: 100vw;
  height: 100vh;
  background: var(--color-window-bg);
  backdrop-filter: blur(20px);
  // border: 1px solid var(--color-border);

  :deep(.n-layout) {
    background: transparent;
  }
}

.header {
  padding: 0;
  background: transparent;
}

.content {
  background: transparent;
  height: calc(100vh - var(--titlebar-height));
}
</style>
