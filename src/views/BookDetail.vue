<template>
  <div class="book-detail">
    <n-button class="back-btn" quaternary circle aria-label="返回" @click="goBack">
      <template #icon>
        <n-icon :component="ArrowLeftOutlined" />
      </template>
    </n-button>

    <section class="detail-header">
      <div class="book-summary">
        <div class="summary-cover">
          <BookCover :title="book?.title ?? ''" :author="book?.author" :cover="book?.cover" />
        </div>

        <div class="summary-meta">
          <h2 class="summary-title">{{ book?.title || "加载中..." }}</h2>

          <n-descriptions :column="2" label-placement="left" size="small">
            <n-descriptions-item label="作者">{{ book?.author || "佚名" }}</n-descriptions-item>
            <n-descriptions-item label="总章节">{{ book?.total_chapters ?? 0 }}</n-descriptions-item>
            <n-descriptions-item label="总字数">{{ formatChars(book?.total_chars) }}</n-descriptions-item>
            <n-descriptions-item label="导入时间">{{ formatTime(book?.create_time) }}</n-descriptions-item>
            <n-descriptions-item label="上次阅读">{{ formatTime(book?.last_read_time) }}</n-descriptions-item>
            <n-descriptions-item label="阅读进度">{{ readProgress }}</n-descriptions-item>
            <n-descriptions-item label="简介" :span="2">
              <n-popover
                trigger="hover"
                placement="bottom-start"
                :show-arrow="true"
                content-class="intro-popover-panel"
                :content-style="{
                  width: '520px',
                  maxWidth: 'calc(100vw - 64px)',
                  maxHeight: '180px',
                  padding: '0',
                  overflow: 'hidden',
                }"
              >
                <template #trigger>
                  <div class="intro-preview">{{ book?.introduction || "暂无简介" }}</div>
                </template>
                <n-scrollbar class="intro-popover-scrollbar" style="height: 180px">
                  <div class="intro-popover">{{ book?.introduction || "暂无简介" }}</div>
                </n-scrollbar>
              </n-popover>
            </n-descriptions-item>
          </n-descriptions>
        </div>
      </div>
    </section>

    <n-divider class="detail-divider" />

    <section class="chapter-section">
      <div class="chapter-header">
        <div class="chapter-header-left">
          <span class="chapter-name">章节列表</span>
          <span class="chapter-count">共 {{ total }} 章</span>
        </div>
        <n-input
          v-model:value="keyword"
          class="chapter-search"
          placeholder="搜索章节"
          clearable
          size="small"
          @update:value="loadChapters"
        />
      </div>

      <n-scrollbar class="chapter-scrollbar">
        <n-spin :show="loadingChapters">
          <n-grid
            v-if="chapterList.length > 0"
            class="chapter-grid"
            :cols="2"
            :x-gap="16"
            :y-gap="6"
            responsive="screen"
          >
            <n-gi v-for="ch in chapterList" :key="ch.id">
              <div
                class="chapter-item"
                :class="{ 'is-read': ch.is_read }"
                @click="readChapter(ch)"
              >
                <span class="chapter-index">{{ ch.number }}</span>
                <span class="chapter-title" :title="ch.title">{{ ch.title }}</span>
                <span class="chapter-chars">{{ ch.total_chars }} 字</span>
              </div>
            </n-gi>
          </n-grid>
          <n-empty
            v-if="!loadingChapters && chapterList.length === 0"
            class="chapter-empty"
            description="暂无章节"
          />
        </n-spin>
      </n-scrollbar>

      <div class="pagination-wrap">
        <n-pagination
          v-model:page="page"
          :page-size="limit"
          :item-count="total"
          :page-slot="7"
          show-size-picker
          :page-sizes="[20, 50, 100]"
          @update:page="onPageChange"
          @update:page-size="onPageSizeChange"
        />
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window'
import { ArrowLeftOutlined } from "@vicons/antd"
import dayjs from "dayjs"
import BookApi from "@/api/book"
import BookCover from "@/components/BookCover.vue"
import { useLoading } from "@/hooks/useLoading"
import type { Books, Chapters } from "@/types/global"

const router = useRouter()
const route = useRoute()
const message = useMessage()
const appWindow = getCurrentWindow()
const { withLoading } = useLoading()

const bookId = computed(() => Number(route.query.id) || 0)

const book = ref<Books | null>(null)
const chapterList = ref<Chapters[]>([])
const total = ref(0)
const page = ref(1)
const limit = ref(50)
const loadingChapters = ref(false)
const keyword = ref("")

const goBack = () => router.back()

const readChapter = (ch: Chapters) => {
  router.push({ name: "Reader", query: { id: ch.id } })
}

const readProgress = computed(() => {
  const b = book.value
  if (!b || b.total_chapters <= 0) return "—"
  const percent = Math.round((b.last_read_chapter_id / b.total_chapters) * 100)
  return `第 ${b.last_read_chapter_id} 章 · ${percent}%`
})

const formatChars = (chars?: number) => {
  if (!chars) return "0 字"
  if (chars >= 10000) return `${(chars / 10000).toFixed(1)} 万字`
  return `${chars} 字`
}

const formatTime = (t?: string | null) => {
  if (!t) return "—"
  const d = dayjs(t)
  return d.isValid() ? d.format("YYYY-MM-DD HH:mm") : t
}

const loadBook = async () => {
  if (!bookId.value) return
  const res = await withLoading(() => BookApi.detail(bookId.value), "加载中...")
  if (res.code === 0) {
    book.value = res.data
  } else {
    message.error(res.msg)
  }
}

const loadChapters = async () => {
  if (!bookId.value) return
  loadingChapters.value = true
  try {
    const res = await withLoading(
      () => BookApi.chapters(bookId.value, keyword.value, page.value, limit.value),
      "加载章节中..."
    )
    if (res.code === 0) {
      chapterList.value = res.data.list
      total.value = res.data.total
    } else {
      message.error(res.msg)
    }
  } catch (e) {
    message.error("章节加载失败")
  } finally {
    loadingChapters.value = false
  }
}

const onPageChange = (p: number) => {
  page.value = p
  loadChapters()
}

const onPageSizeChange = (size: number) => {
  limit.value = size
  page.value = 1
  loadChapters()
}

onMounted(() => {
  loadBook()
  loadChapters()
  appWindow.setSize(new LogicalSize(800, 600))
})
</script>

<style lang="scss" scoped>
.book-detail {
  position: relative;
  box-sizing: border-box;
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  padding: 18px 24px 16px;
}

.back-btn {
  position: absolute;
  top: 8px;
  left: 24px;
  z-index: 10;
  background-color: var(--color-surface);
  box-shadow: var(--shadow-card);
  transition: box-shadow 0.2s ease, background-color 0.2s ease;

  &:hover {
    box-shadow: var(--shadow-card-hover);
  }
}

.detail-header {
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.book-summary {
  display: flex;
  gap: 20px;
}

.summary-cover {
  width: 120px;
  height: 180px;
  flex-shrink: 0;
  align-self: flex-start;
  border-radius: var(--radius-sm);
  overflow: hidden;
  box-shadow: var(--shadow-card);
}

.summary-meta {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.summary-title {
  margin: 0;
  font-size: 20px;
  font-weight: 700;
  color: var(--color-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.intro-preview {
  line-height: 1.6;
  font-size: 13px;
  color: var(--color-text-secondary);
  word-break: break-all;
  white-space: pre-wrap;
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 2;
  overflow: hidden;
  cursor: help;
}

:global(.intro-popover-scrollbar .n-scrollbar-container) {
  height: 100%;
}

:global(.intro-popover) {
  box-sizing: border-box;
  padding: 12px 8px 12px 12px;
  color: var(--color-text-primary);
  font-size: 13px;
  line-height: 1.7;
  word-break: break-all;
  white-space: pre-wrap;
}

.detail-divider {
  flex-shrink: 0;
  margin: 14px 0;
}

.chapter-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.chapter-header {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  margin-bottom: 8px;
}

.chapter-header-left {
  display: flex;
  align-items: baseline;
  gap: 8px;
  min-width: 0;
}

.chapter-search {
  width: 220px;
  flex-shrink: 0;
}

.chapter-name {
  font-size: 15px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.chapter-count {
  font-size: 12px;
  color: var(--color-text-secondary);
}

.chapter-scrollbar {
  flex: 1;
  min-height: 0;

  :deep(.n-scrollbar-container) {
    height: 100%;
  }

  :deep(.n-scrollbar-content) {
    min-height: 100%;
  }
}

.chapter-item {
  display: flex;
  align-items: center;
  gap: 10px;
  box-sizing: border-box;
  width: 100%;
  padding: 8px 10px;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: background-color 0.2s ease;

  &:hover {
    background-color: var(--color-surface-hover);
  }

  &.is-read {
    opacity: 0.45;

    .chapter-title,
    .chapter-index,
    .chapter-chars {
      color: var(--color-text-secondary);
    }
  }
}

.chapter-index {
  flex-shrink: 0;
  min-width: 32px;
  color: var(--color-text-secondary);
  font-size: 13px;
}

.chapter-title {
  flex: 1;
  min-width: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  color: var(--color-text-primary);
  font-size: 14px;
}

.chapter-chars {
  flex-shrink: 0;
  color: var(--color-text-secondary);
  font-size: 12px;
}

.chapter-empty {
  padding: 40px 0;
}

.pagination-wrap {
  flex-shrink: 0;
  display: flex;
  justify-content: center;
  padding-top: 12px;
}
</style>
