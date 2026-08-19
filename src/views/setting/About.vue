<template>
  <div class="setting-page">
    <h2 class="page-title">关于</h2>
    <p class="page-desc">关于本应用的信息</p>

    <div class="about-card">
      <div class="app-logo">幽</div>
      <div class="app-name">幽灵阅读器</div>
      <div class="app-version">版本 0.1.0</div>
      <div class="app-desc">一款安静、沉浸的本地阅读器</div>
    </div>
  </div>
</template>

<script setup lang="ts"></script>

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

.about-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 320px;
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-card);
  box-shadow: var(--shadow-card);
  margin-top: 8px;
}

.app-logo {
  width: 64px;
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-card);
  background: var(--color-accent-gradient);
  color: var(--color-text-invert);
  font-size: 28px;
  font-weight: 700;
  margin-bottom: 16px;
}

.app-name {
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.app-version {
  font-size: 13px;
  color: var(--color-text-secondary);
  margin-top: 4px;
}

.app-desc {
  font-size: 13px;
  color: var(--color-text-secondary);
  margin-top: 16px;
}
</style>
