<template>
  <transition name="fade">
    <div v-if="loading" class="global-loading">
      <div class="global-loading-box">
        <n-spin size="large" />
        <span class="global-loading-text">{{ text }}</span>
      </div>
    </div>
  </transition>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { useLoadingStore } from "@/stores/loading"

const store = useLoadingStore()
const loading = computed(() => store.loading)
const text = computed(() => store.text)
</script>

<style lang="scss" scoped>
.global-loading {
  position: fixed;
  inset: 0;
  z-index: 3000;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.2);
  backdrop-filter: blur(2px);
}

.global-loading-box {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 14px;
  padding: 28px 36px;
  border-radius: var(--radius-card);
  background: var(--color-surface);
  box-shadow: var(--shadow-card-hover);
}

.global-loading-text {
  font-size: 14px;
  color: var(--color-text-secondary);
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
