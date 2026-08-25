import { ref } from "vue"
import { check, type Update, type DownloadEvent } from "@tauri-apps/plugin-updater"
import { relaunch } from "@tauri-apps/plugin-process"

export type CheckPromptResult = "none" | "error" | "prompting"

// 当前待更新的 Update 对象（非响应式，避免代理破坏资源句柄）
let currentUpdate: Update | null = null

export const checking = ref(false)
export const downloading = ref(false)
export const installing = ref(false)
export const showUpdateModal = ref(false)
export const progress = ref(0)
export const downloadedBytes = ref(0)
export const totalBytes = ref(0)

export const updateInfo = ref<{
  currentVersion: string
  version: string
  date?: string
  body?: string
} | null>(null)

const SKIPPED_KEY = "skippedUpdateVersion"

function getSkippedVersion(): string | null {
  try {
    return localStorage.getItem(SKIPPED_KEY)
  } catch {
    return null
  }
}

function setSkippedVersion(version: string) {
  try {
    localStorage.setItem(SKIPPED_KEY, version)
  } catch {
    // ignore
  }
}

export function formatBytes(bytes: number): string {
  if (bytes <= 0) return "0 B"
  const units = ["B", "KB", "MB", "GB"]
  let v = bytes
  let i = 0
  while (v >= 1024 && i < units.length - 1) {
    v /= 1024
    i++
  }
  return `${v.toFixed(v >= 10 || i === 0 ? 0 : 1)} ${units[i]}`
}

/**
 * 检查更新，若有可用更新则展示更新弹窗
 * @returns none=无更新/已跳过；error=检查出错；prompting=已展示弹窗等待用户操作
 */
export async function checkAndPrompt(): Promise<CheckPromptResult> {
  if (checking.value || downloading.value || installing.value) return "none"
  checking.value = true
  try {
    const update = await check()
    if (!update) return "none"
    if (getSkippedVersion() === update.version) return "none"

    currentUpdate = update
    updateInfo.value = {
      currentVersion: update.currentVersion,
      version: update.version,
      date: update.date,
      body: update.body,
    }
    progress.value = 0
    downloadedBytes.value = 0
    totalBytes.value = 0
    showUpdateModal.value = true
    return "prompting"
  } catch (error) {
    console.error("检查更新失败:", error)
    return "error"
  } finally {
    checking.value = false
  }
}

/**
 * 开始下载并安装更新（带进度回调），完成后重启应用
 */
export async function startUpdate(): Promise<void> {
  if (!currentUpdate || downloading.value || installing.value) return
  downloading.value = true
  progress.value = 0
  downloadedBytes.value = 0
  totalBytes.value = 0
  try {
    await currentUpdate.download((event: DownloadEvent) => {
      if (event.event === "Started") {
        totalBytes.value = event.data.contentLength ?? 0
      } else if (event.event === "Progress") {
        downloadedBytes.value += event.data.chunkLength
        if (totalBytes.value > 0) {
          progress.value = Math.min(
            100,
            Math.round((downloadedBytes.value / totalBytes.value) * 100)
          )
        }
      }
    })
    downloading.value = false
    installing.value = true
    await currentUpdate.install()
    await relaunch()
  } catch (error) {
    console.error("更新失败:", error)
    downloading.value = false
    installing.value = false
    throw error
  }
}

/** 跳过当前版本：记录已跳过版本并关闭弹窗 */
export function skipVersion() {
  if (currentUpdate) {
    setSkippedVersion(currentUpdate.version)
  }
  currentUpdate = null
  updateInfo.value = null
  showUpdateModal.value = false
}

/** 取消更新：关闭弹窗 */
export function cancelUpdate() {
  currentUpdate = null
  updateInfo.value = null
  showUpdateModal.value = false
}
