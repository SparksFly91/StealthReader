<template>
  <div class="glass-panel" :style="readerStyle" data-tauri-drag-region @mousedown="onWindowMouseDown">
    <div class="reading-wrap">
      <div v-if="currentPage == 0" class="chapter-name">{{ chapter?.title || "加载中..." }}</div>

      <div ref="pageEl" class="page-body" @click="onPageClick">
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
    for (let i = 0; i < para.length; i += charsPerLine) {
      logicalLines.push(para.slice(i, i + charsPerLine))
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

const onPageClick = (e: MouseEvent) => {
  const el = e.currentTarget as HTMLElement
  const rect = el.getBoundingClientRect()
  if (e.clientX - rect.left < rect.width / 2) prevPage()
  else nextPage()
}

const onWindowMouseDown = (e: MouseEvent) => {
  const target = e.target as HTMLElement | null
  if (target?.closest("button, a, input, textarea, select, [role='button']")) return
  void appWindow.startDragging()
}

const onKeydown = (e: KeyboardEvent) => {
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
})
</script>

<style scoped>
.glass-panel {
  position: relative;
  box-sizing: border-box;
  width: 100vw;
  height: 100vh;
  background: var(--reader-background-color);
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

.page-body {
  flex: 1;
  min-height: 0;
  cursor: pointer;
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
</style>
