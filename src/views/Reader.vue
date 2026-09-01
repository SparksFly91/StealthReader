<template>
  <div
    class="glass-panel"
    :style="readerStyle"
    data-tauri-drag-region
    @mousedown="onWindowMouseDown"
    @contextmenu.prevent="onContextMenu"
  >
    <div class="reading-wrap">
      <div class="chapter-name" :class="{ 'chapter-name--hidden': currentPage !== 0 }">
        {{ chapter?.title || "加载中..." }}
      </div>

      <div ref="pageEl" class="page-body">
        <div class="page-text">{{ currentText }}</div>
      </div>

      <div class="page-nav">
        <span class="page-info">{{ pageInfo }}</span>
        <div class="nav-btns">
          <n-space class="nav-group nav-group--left" :size="0">
            <n-button size="small" text @click="switchChapter(-1)">上一章</n-button>
            <n-button size="small" text :disabled="currentPage <= 0" @click="prevPage">上一页</n-button>
          </n-space>
          <n-button class="nav-back" size="small" text @click="goBack">返回</n-button>
          <n-space class="nav-group nav-group--right" :size="0">
            <n-button size="small" text :disabled="currentPage >= pages.length - 1" @click="nextPage">
              下一页
            </n-button>
            <n-button size="small" text @click="switchChapter(1)">下一章</n-button>
          </n-space>
        </div>
      </div>
    </div>

    <n-popover
      v-model:show="menuShow"
      trigger="manual"
      :x="menuX"
      :y="menuY"
      placement="bottom-start"
      :show-arrow="false"
      @clickoutside="menuShow = false"
    >
      <div class="ctx-menu" @contextmenu.prevent>
        <div class="ctx-item">
          <span class="ctx-label">字体颜色</span>
          <div class="color-picker-control">
            <n-color-picker v-model:value="menuFontColor" show-alpha :modes="['hex']" placement="bottom-end">
              <template #trigger="{ value, onClick, ref: triggerRef }">
                <div
                  :ref="triggerRef"
                  class="color-swatch"
                  :style="{ backgroundColor: value || '#000000' }"
                  role="button"
                  tabindex="0"
                  aria-label="选择字体颜色"
                  @click="onClick"
                  @keydown.enter="onClick"
                  @keydown.space.prevent="onClick"
                />
              </template>
            </n-color-picker>
            <div class="color-values">
              <span>{{ getColorDetails(menuFontColor).color }}</span>
              <small>透明度 {{ getColorDetails(menuFontColor).opacity }}%</small>
            </div>
          </div>
        </div>
        <div class="ctx-item">
          <span class="ctx-label">背景颜色</span>
          <div class="color-picker-control">
            <n-color-picker v-model:value="menuBackgroundColor" show-alpha :modes="['hex']" placement="bottom-end">
              <template #trigger="{ value, onClick, ref: triggerRef }">
                <div
                  :ref="triggerRef"
                  class="color-swatch"
                  :style="{ backgroundColor: value || '#ffffff' }"
                  role="button"
                  tabindex="0"
                  aria-label="选择背景颜色"
                  @click="onClick"
                  @keydown.enter="onClick"
                  @keydown.space.prevent="onClick"
                />
              </template>
            </n-color-picker>
            <div class="color-values">
              <span>{{ getColorDetails(menuBackgroundColor).color }}</span>
              <small>透明度 {{ getColorDetails(menuBackgroundColor).opacity }}%</small>
            </div>
          </div>
        </div>
      </div>
    </n-popover>
  </div>
</template>

<script setup lang="ts">
import { useSettingStore } from '@/stores/setting'
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window'
import BookApi from "@/api/book"
import { useLoading } from "@/hooks/useLoading"
import type { Chapters } from "@/types/global"

const route = useRoute()
const router = useRouter()
const message = useMessage()
const settingStore = useSettingStore()
const appWindow = getCurrentWindow()
const { withLoading } = useLoading()

const chapterId = computed(() => Number(route.query.id) || 0)
const chapter = ref<Chapters | null>(null)

const pageEl = ref<HTMLElement | null>(null)
const pages = ref<string[]>([])
const currentPage = ref(0)

const goBack = () => router.back()

const currentText = computed(() => pages.value[currentPage.value] ?? "")

const readerStyle = computed(() => ({
  "--reader-font-size": `${settingStore.reader.fontSize}px`,
  "--reader-font-color": settingStore.reader.fontColor,
  "--reader-background-color": settingStore.reader.backgroundColor,
}))

const pageInfo = computed(() => {
  if (pages.value.length === 0) return ""
  return `第 ${currentPage.value + 1} / ${pages.value.length} 页`
})

const measureCharWidth = (fontSize: number) => {
  const probe = document.createElement("span")
  probe.textContent = "国"
  probe.style.cssText = `font-size:${fontSize}px;position:absolute;visibility:hidden;white-space:nowrap;`
  document.body.appendChild(probe)
  const width = probe.getBoundingClientRect().width
  document.body.removeChild(probe)
  return width || fontSize
}

const paginate = () => {
  const content = chapter.value?.content || ""
  if (!content) {
    pages.value = []
    currentPage.value = 0
    return
  }
  const el = pageEl.value
  if (!el) return

  const fontSize = Math.max(1, Number(settingStore.reader.fontSize) || 12)
  const lineHeight = 1.8
  const width = el.clientWidth
  const height = el.clientHeight
  const charWidth = measureCharWidth(fontSize)
  const charsPerLine = Math.max(1, Math.floor(width / charWidth))
  const linesPerPage = Math.max(1, Math.floor(height / (fontSize * lineHeight)))

  // 把内容按行数拆分成逻辑行，再按每页行数分组
  const logicalLines: string[] = []
  for (const para of content.split(/\r?\n/)) {
    if (para === "") {
      logicalLines.push("")
      continue
    }
    // 段落首行缩进两格（使用全角空格）
    const indented = "　　" + para
    for (let i = 0; i < indented.length; i += charsPerLine) {
      logicalLines.push(indented.slice(i, i + charsPerLine))
    }
  }

  const arr: string[] = []
  for (let i = 0; i < logicalLines.length; i += linesPerPage) {
    arr.push(logicalLines.slice(i, i + linesPerPage).join("\n"))
  }
  pages.value = arr.length ? arr : [""]
  currentPage.value = 0
}

const prevPage = () => {
  if (currentPage.value > 0) currentPage.value--
}

const nextPage = () => {
  if (currentPage.value < pages.value.length - 1) currentPage.value++
}

// 右键快捷设置菜单（字体颜色 / 背景颜色）
const menuShow = ref(false)
const menuX = ref(0)
const menuY = ref(0)
const menuFontColor = ref(settingStore.reader.fontColor)
const menuBackgroundColor = ref(settingStore.reader.backgroundColor)

const getColorDetails = (value: string | null) => {
  const color = value || "#000000"
  const match =
    color.match(/^#([\da-f]{6})([\da-f]{2})$/i) ||
    color.match(/^#([\da-f]{3})([\da-f])$/i)
  const alphaMatch = color.match(/^rgba\(\s*\d+\s*,\s*\d+\s*,\s*\d+\s*,\s*([\d.]+)\s*\)$/i)
  const alpha = match
    ? Math.round((parseInt(match[2].length === 1 ? match[2] + match[2] : match[2], 16) / 255) * 100)
    : alphaMatch
      ? Math.round(Number(alphaMatch[1]) * 100)
      : 100

  return {
    color: match ? `#${match[1].toUpperCase()}` : color,
    opacity: Math.min(100, Math.max(0, alpha)),
  }
}

const onContextMenu = (e: MouseEvent) => {
  menuX.value = e.clientX
  menuY.value = e.clientY
  menuShow.value = true
}

watch(menuFontColor, (v) => {
  if (v) settingStore.reader.fontColor = v
})
watch(menuBackgroundColor, (v) => {
  if (v) settingStore.reader.backgroundColor = v
})

const onWindowMouseDown = (e: MouseEvent) => {
  if (e.button !== 0) return
  const target = e.target as HTMLElement | null
  if (target?.closest("button, a, input, textarea, select, [role='button']")) return
  void appWindow.startDragging()
}

const onKeydown = (e: KeyboardEvent) => {
  if (e.key === "Escape") {
    menuShow.value = false
    return
  }
  if (e.key === "ArrowLeft") {
    e.preventDefault()
    switchChapter(-1)
  } else if (e.key === "ArrowRight") {
    e.preventDefault()
    switchChapter(1)
  } else if (e.key === "ArrowUp" || e.key === "PageUp") {
    e.preventDefault()
    prevPage()
  } else if (e.key === "ArrowDown" || e.key === "PageDown" || e.key === " ") {
    e.preventDefault()
    nextPage()
  }
}

const onResize = () => paginate()

watch(
  () => settingStore.reader.fontSize,
  () => nextTick(paginate)
)

const loadChapter = async () => {
  if (!chapterId.value) {
    message.error("章节参数错误")
    return
  }
  const res = await withLoading(() => BookApi.chapterDetail(chapterId.value), "加载中...")
  if (res.code === 0) {
    chapter.value = res.data
    nextTick(paginate)
  } else {
    message.error(res.msg)
  }
}

const switchChapter = async (offset: number) => {
  const cur = chapter.value
  if (!cur) return
  const res = await withLoading(
    () => BookApi.chapterNav(cur.book_id, cur.number, offset),
    "加载中..."
  )
  if (res.code === 0) {
    if (res.data) {
      chapter.value = res.data
      nextTick(paginate)
    } else {
      message.info(offset > 0 ? "已经是最后一章了" : "已经是第一章了")
    }
  } else {
    message.error(res.msg)
  }
}

onMounted(() => {
  window.addEventListener("keydown", onKeydown)
  window.addEventListener("resize", onResize)
  appWindow.setShadow(settingStore.reader.showShadow)
  appWindow.setSize(new LogicalSize(settingStore.reader.windowWidth, settingStore.reader.windowHeight))
  loadChapter()
})

onUnmounted(() => {
  window.removeEventListener("keydown", onKeydown)
  window.removeEventListener("resize", onResize)
  // 离开阅读模式时，恢复主窗口的阴影设置
  appWindow.setShadow(settingStore.appearance.showShadow)
})
</script>

<style scoped>
.glass-panel {
  position: relative;
  box-sizing: border-box;
  width: 100vw;
  height: 100vh;
  background: var(--reader-background-color);
  border-radius: var(--radius-window);
  /* border: 1px solid rgba(255, 255, 255, 0.2); */
  padding: 10px;
  color: var(--reader-font-color);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  user-select: none;
  -webkit-user-select: none;
}

.back-btn {
  position: absolute;
  top: 12px;
  left: 16px;
  z-index: 10;
  background: transparent;
  transition: background-color 0.2s ease;

  &:hover {
    background: rgba(0, 0, 0, 0.06);
  }
}

.reading-wrap {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  width: 100%;
  max-width: 720px;
  margin: 0 auto;
  /* padding: 44px 40px 16px; */
}

.chapter-name {
  flex-shrink: 0;
  text-align: center;
  font-size: 14px;
  font-weight: 700;
  color: var(--reader-font-color);
  margin-bottom: 20px;
}

.chapter-name--hidden {
  visibility: hidden;
}

.page-body {
  flex: 1;
  min-height: 0;
}

.page-text {
  height: 100%;
  overflow: hidden;
  font-size: var(--reader-font-size);
  line-height: 1.8;
  color: var(--reader-font-color);
  white-space: pre-wrap;
  word-break: break-word;
  text-align: justify;
}

.page-nav {
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  margin-top: 8px;
  padding-bottom: 2px;
}

.page-info {
  font-size: 12px;
  color: var(--reader-font-color);
  opacity: 0.65;
}

.glass-panel :deep(.n-button) {
  color: var(--reader-font-color);
}

.nav-btns {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto minmax(0, 1fr);
  align-items: center;
  gap: 2px;
  width: 100%;
}

.nav-group {
  min-width: 0;
  overflow: hidden;
}

.nav-group--right {
  justify-content: flex-end;
}

.nav-btns :deep(.n-button) {
  min-width: 0;
  padding: 0 3px;
  font-size: 12px;
  white-space: nowrap;
}

.nav-back {
  padding: 0 4px;
}

.ctx-menu {
  display: flex;
  flex-direction: column;
  gap: 12px;
  min-width: 176px;
}

.ctx-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.ctx-label {
  font-size: 13px;
  color: var(--color-text-primary);
  white-space: nowrap;
}

.color-picker-control {
  display: inline-flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 6px;
}

.color-swatch {
  width: 22px;
  height: 22px;
  border: 2px solid #ffffff;
  border-radius: 50%;
  cursor: pointer;
  background-position: 0 0, 0 4px, 4px -4px, -4px 0;
  background-size: 8px 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.color-swatch:focus-visible {
  outline: 2px solid var(--color-accent);
  outline-offset: 2px;
}

.color-values {
  display: flex;
  flex-direction: column;
  gap: 1px;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
  font-size: 12px;
  line-height: 16px;
}

.color-values small {
  color: var(--color-text-secondary);
  font-family: inherit;
  font-size: 11px;
}
</style>
