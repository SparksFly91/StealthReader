<template>
  <div class="setting-page">
    <h2 class="page-title">关于</h2>
    <p class="page-desc">关于幽灵阅读器</p>

    <div class="about-hero">
      <div class="about-logo">
        <img :src="appIcon" alt="幽灵阅读器" />
      </div>
      <div class="about-info">
        <div class="about-name">幽灵阅读器</div>
        <div class="about-subname">Stealth Reader</div>
        <div class="about-meta">
          <div class="about-version">版本 {{ version || "0.0.0" }}</div>
          <n-button
            class="check-update-btn"
            size="tiny"
            secondary
            round
            :loading="checkingUpdate"
            @click="onCheckUpdate"
          >
            <template #icon>
              <n-icon :component="SyncOutlined" />
            </template>
            {{ checkingUpdate ? "检查中..." : "检查更新" }}
          </n-button>
        </div>
      </div>
    </div>

    <div class="about-intro">
      一款安静、本地优先的桌面小说阅读器。轻量、纯本地、无账号、无广告，专注「好好读一本书」。
    </div>

    <div class="about-section">
      <div class="section-title">核心特性</div>
      <div class="feature-list">
        <div v-for="f in features" :key="f.title" class="feature-item">
          <span class="feature-icon" :style="{ backgroundColor: f.bg }">
            <n-icon :component="f.icon" :size="16" />
          </span>
          <div class="feature-body">
            <div class="feature-title">{{ f.title }}</div>
            <div class="feature-desc">{{ f.desc }}</div>
          </div>
        </div>
      </div>
    </div>

    <div class="about-section">
      <div class="section-title">技术栈</div>
      <div class="tech-list">
        <div v-for="t in tech" :key="t.name" class="tech-item">
          <span class="tech-name">{{ t.name }}</span>
          <span class="tech-desc">{{ t.desc }}</span>
        </div>
      </div>
    </div>

    <div class="about-footer">
      <span>© {{ year }} Stealth Reader</span>
      <span>Apache-2.0 License</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getVersion } from "@tauri-apps/api/app"
import {
  BookOutlined,
  FileTextOutlined,
  FontColorsOutlined,
  LockOutlined,
  ScanOutlined,
  SyncOutlined,
  ThunderboltOutlined,
} from "@vicons/antd"
import appIcon from "../../../app-icon.svg"
import { checkAndPrompt } from "@/utils/updater"

const message = useMessage()
const year = new Date().getFullYear()
const version = ref("")
const checkingUpdate = ref(false)

getVersion().then((v) => {
  version.value = v
})

const onCheckUpdate = async () => {
  if (checkingUpdate.value) return
  checkingUpdate.value = true
  try {
    const result = await checkAndPrompt()
    if (result === "none") {
      message.success("当前已是最新版本")
    } else if (result === "error") {
      message.error("检查更新失败，请稍后重试")
    }
    // prompting：已展示更新弹窗，由弹窗处理后续流程
  } finally {
    checkingUpdate.value = false
  }
}

const features = [
  {
    title: "本地优先",
    desc: "纯本地存储，数据不离开你的设备",
    icon: LockOutlined,
    bg: "color-mix(in srgb, #667eea 14%, transparent)",
  },
  {
    title: "智能分章",
    desc: "自动识别「第X章」「Chapter N」等章节格式",
    icon: ScanOutlined,
    bg: "color-mix(in srgb, #10b981 14%, transparent)",
  },
  {
    title: "自动编码检测",
    desc: "GBK / UTF-8 等中文编码自动识别",
    icon: FileTextOutlined,
    bg: "color-mix(in srgb, #f59e0b 14%, transparent)",
  },
  {
    title: "沉浸阅读",
    desc: "可拖拽窗口、透明背景、翻页阅读",
    icon: BookOutlined,
    bg: "color-mix(in srgb, #ec4899 14%, transparent)",
  },
  {
    title: "外观自定义",
    desc: "字体大小 / 颜色 / 背景随心调整",
    icon: FontColorsOutlined,
    bg: "color-mix(in srgb, #8b5cf6 14%, transparent)",
  },
  {
    title: "轻量快速",
    desc: "Tauri 2 构建，内存占用远低于 Electron",
    icon: ThunderboltOutlined,
    bg: "color-mix(in srgb, #06b6d4 14%, transparent)",
  },
]

const tech = [
  { name: "桌面框架", desc: "Tauri 2 + Rust" },
  { name: "前端框架", desc: "Vue 3 + TypeScript" },
  { name: "UI 组件", desc: "Naive UI" },
  { name: "数据存储", desc: "SQLite (sqlx)" },
  { name: "编码检测", desc: "chardetng" },
]
</script>

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

.about-hero {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 20px 16px;
  border-top: 1px solid var(--color-border);
  border-bottom: 1px solid var(--color-border);
}

.about-logo {
  width: 64px;
  height: 64px;
  flex-shrink: 0;
  border-radius: 16px;
  overflow: hidden;
  box-shadow: var(--shadow-card);

  img {
    width: 100%;
    height: 100%;
    display: block;
  }
}

.about-info {
  min-width: 0;
}

.about-name {
  font-size: 18px;
  font-weight: 700;
  color: var(--color-text-primary);
}

.about-subname {
  font-size: 13px;
  color: var(--color-text-secondary);
  margin-top: 1px;
}

.about-version {
  display: inline-block;
  margin-top: 6px;
  padding: 1px 8px;
  border-radius: 999px;
  background-color: color-mix(in srgb, var(--color-accent) 12%, transparent);
  color: var(--color-accent);
  font-size: 12px;
  font-weight: 600;
}

.about-meta {
  display: flex;
  align-items: center;
  gap: 10px;
}

.check-update-btn {
  margin-top: 6px;
  font-weight: 500;

  :deep(.n-button__icon) {
    margin-right: 4px;
  }
}

.about-intro {
  margin: 18px 0;
  padding: 14px 16px;
  border-radius: var(--radius-sm);
  background-color: var(--color-window-bg);
  font-size: 13px;
  line-height: 1.7;
  color: var(--color-text-secondary);
}

.about-section {
  & + & {
    margin-top: 20px;
  }
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: 10px;
}

.feature-list {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 10px;
}

.feature-item {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  padding: 12px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  transition: border-color 0.2s ease, box-shadow 0.2s ease;

  &:hover {
    border-color: var(--color-border-hover);
    box-shadow: var(--shadow-card);
  }
}

.feature-icon {
  width: 32px;
  height: 32px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  color: var(--color-text-primary);
}

.feature-body {
  min-width: 0;
}

.feature-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.feature-desc {
  margin-top: 2px;
  font-size: 12px;
  line-height: 1.5;
  color: var(--color-text-secondary);
}

.tech-list {
  overflow: hidden;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
}

.tech-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 10px 14px;
  min-height: 40px;

  & + & {
    border-top: 1px solid var(--color-border);
  }
}

.tech-name {
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.tech-desc {
  font-size: 13px;
  color: var(--color-text-secondary);
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
}

.about-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  margin-top: 24px;
  padding-top: 14px;
  border-top: 1px solid var(--color-border);
  font-size: 12px;
  color: var(--color-text-secondary);
}

@media (max-width: 640px) {
  .setting-page {
    padding: 18px 16px 24px;
  }

  .feature-list {
    grid-template-columns: 1fr;
  }

  .about-footer {
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
  }
}
</style>
