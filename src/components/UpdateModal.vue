<template>
  <n-modal :show="showUpdateModal" preset="card" :title="modalTitle" :bordered="false" :closable="!busy"
    :mask-closable="false" :close-on-esc="false" style="width: 430px" @close="cancelUpdate">
    <template #header-extra>
      <span v-if="updateInfo" class="update-current">当前 v{{ updateInfo.currentVersion }}</span>
    </template>

    <div class="update-body">
      <n-scrollbar class="update-scroll">
        <div v-if="renderedBody" class="markdown-body" v-html="renderedBody" />
        <n-empty v-else description="暂无更新说明" />
      </n-scrollbar>
    </div>

    <div v-if="downloading || installing" class="update-progress">
      <n-progress type="line" :percentage="progress" :processing="downloading" indicator-placement="inside"
        :height="18" />
      <div class="progress-hint">{{ progressHint }}</div>
    </div>

    <template #footer>
      <div class="update-footer">
        <n-button quaternary :disabled="busy" @click="skipVersion">跳过此版本</n-button>
        <div class="update-footer-right">
          <n-button v-if="!busy" @click="cancelUpdate">取消</n-button>
          <n-button type="primary" :loading="downloading" @click="startUpdate">
            {{ installing ? "安装中..." : downloading ? "下载中..." : "立即更新" }}
          </n-button>
        </div>
      </div>
    </template>
  </n-modal>
</template>

<script setup lang="ts">
import { computed } from "vue"
import MarkdownIt from "markdown-it"
import {
  showUpdateModal,
  updateInfo,
  downloading,
  installing,
  progress,
  downloadedBytes,
  totalBytes,
  formatBytes,
  startUpdate,
  skipVersion,
  cancelUpdate,
} from "@/utils/updater"

const md = new MarkdownIt({ html: false, linkify: true })

const busy = computed(() => downloading.value || installing.value)

const modalTitle = computed(() =>
  updateInfo.value ? `发现新版本 v${updateInfo.value.version}` : "发现新版本"
)

const renderedBody = computed(() => md.render(updateInfo.value?.body || ""))

const progressHint = computed(() => {
  if (installing.value) return "正在安装更新，即将重启应用..."
  if (downloading.value) {
    if (totalBytes.value > 0) {
      return `正在下载更新 ${progress.value}% · ${formatBytes(downloadedBytes.value)} / ${formatBytes(totalBytes.value)}`
    }
    return `正在下载更新... ${formatBytes(downloadedBytes.value)}`
  }
  return ""
})
</script>

<style lang="scss" scoped>
.update-current {
  font-size: 12px;
  color: var(--color-text-secondary);
}

.update-body {
  .update-scroll {
    height: 340px;
  }
}

.markdown-body {
  padding-right: 8px;
  font-size: 13px;
  line-height: 1.7;
  color: var(--color-text-primary);
  word-break: break-word;

  :deep(h1),
  :deep(h2),
  :deep(h3),
  :deep(h4),
  :deep(h5),
  :deep(h6) {
    margin: 14px 0 8px;
    font-weight: 600;
    line-height: 1.4;
    color: var(--color-text-primary);
  }

  :deep(h1) {
    font-size: 18px;
  }

  :deep(h2) {
    font-size: 16px;
  }

  :deep(h3) {
    font-size: 14px;
  }

  :deep(h4),
  :deep(h5),
  :deep(h6) {
    font-size: 13px;
  }

  :deep(p) {
    margin: 6px 0;
  }

  :deep(ul),
  :deep(ol) {
    margin: 6px 0;
    padding-left: 20px;
  }

  :deep(li) {
    margin: 2px 0;
  }

  :deep(strong) {
    font-weight: 600;
  }

  :deep(code) {
    padding: 1px 5px;
    border-radius: 4px;
    background: var(--color-surface-hover);
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 12px;
  }

  :deep(pre) {
    margin: 8px 0;
    padding: 10px 12px;
    border-radius: 6px;
    background: var(--color-surface-hover);
    overflow-x: auto;

    code {
      padding: 0;
      background: transparent;
    }
  }

  :deep(a) {
    color: var(--color-accent);
    text-decoration: none;
  }

  :deep(blockquote) {
    margin: 8px 0;
    padding: 4px 12px;
    border-left: 3px solid var(--color-border-hover);
    color: var(--color-text-secondary);
  }

  :deep(hr) {
    border: none;
    border-top: 1px solid var(--color-border);
    margin: 10px 0;
  }

  :deep(table) {
    width: 100%;
    margin: 8px 0;
    border-collapse: collapse;
    font-size: 12px;
  }

  :deep(th),
  :deep(td) {
    padding: 6px 10px;
    border: 1px solid var(--color-border);
    text-align: left;
  }

  :deep(th) {
    background: var(--color-surface-hover);
    font-weight: 600;
  }
}

.update-progress {
  margin-top: 12px;
}

.progress-hint {
  margin-top: 8px;
  font-size: 12px;
  color: var(--color-text-secondary);
  text-align: center;
}

.update-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  width: 100%;
}

.update-footer-right {
  display: flex;
  align-items: center;
  gap: 8px;
}
</style>
