import { check, type Update } from "@tauri-apps/plugin-updater"
import { ask } from "@tauri-apps/plugin-dialog"
import { relaunch } from "@tauri-apps/plugin-process"

/**
 * 更新检查结果
 * - updated：已确认更新并开始安装（随后应用重启）
 * - none：当前已是最新版本
 * - cancelled：存在更新但用户取消
 * - error：检查或安装过程出错
 */
export type CheckUpdateResult = "updated" | "none" | "cancelled" | "error"

/**
 * 仅检查是否有可用更新，返回 Update 对象或 null
 */
export async function checkUpdate(): Promise<Update | null> {
  return await check()
}

/**
 * 询问用户是否立即更新，确认后下载安装并重启应用
 * @param update 更新对象
 * @returns true=已确认并开始安装；false=用户取消
 */
export async function promptAndInstall(update: Update): Promise<boolean> {
  const yes = await ask(
    `发现新版本 ${update.version}，是否现在更新？\n\n${update.body || ""}`,
    { title: "发现更新", kind: "info" }
  )
  if (!yes) return false

  await update.downloadAndInstall()
  await relaunch()
  return true
}

/**
 * 一键检查并更新（完整流程）。
 * 适合应用启动时静默调用：无更新、用户取消或出错均不打扰用户。
 * @returns 更新结果状态
 */
export async function checkAndUpdate(): Promise<CheckUpdateResult> {
  try {
    const update = await check()
    if (!update) return "none"

    const installed = await promptAndInstall(update)
    return installed ? "updated" : "cancelled"
  } catch (error) {
    console.error("检查更新失败:", error)
    return "error"
  }
}
