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

.about-card {
  display: grid;
  grid-template-columns: 52px minmax(0, 1fr);
  column-gap: 14px;
  align-items: center;
  min-height: 108px;
  padding: 18px 16px;
  border-top: 1px solid var(--color-border);
  border-bottom: 1px solid var(--color-border);
}

.app-logo {
  width: 52px;
  height: 52px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  background: var(--color-accent-gradient);
  color: var(--color-text-invert);
  font-size: 22px;
  font-weight: 700;
  grid-row: span 2;
}

.app-name {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.app-version {
  font-size: 13px;
  color: var(--color-text-secondary);
  margin: 2px 0 0;
}

.app-desc {
  grid-column: 1 / -1;
  font-size: 13px;
  color: var(--color-text-secondary);
  margin: 12px 0 0;
}

@media (max-width: 640px) {
  .setting-page {
    padding: 18px 16px 24px;
  }

  .app-desc {
    margin-left: 0;
  }
}
</style>
