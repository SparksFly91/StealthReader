<template>
  <div class="book-detail">
    <n-button class="back-btn" quaternary circle aria-label="返回" @click="goBack">
      <template #icon>
        <n-icon :component="ArrowLeftOutlined" />
      </template>
    </n-button>

    <n-scrollbar class="page-scrollbar">
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
              <n-scrollbar class="intro-scrollbar">
                <div class="intro-content">{{ book?.introduction || "暂无简介" }}</div>
              </n-scrollbar>
            </n-descriptions-item>
          </n-descriptions>
        </div>
      </div>
    </section>

    <n-divider class="detail-divider" />

    <section class="chapter-section">
      <div class="chapter-header">
        <span class="chapter-name">章节列表</span>
        <span class="chapter-count">共 {{ total }} 章</span>
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
              <div class="chapter-item">
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
    </n-scrollbar>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { ArrowLeftOutlined } from "@vicons/antd"
import dayjs from "dayjs"
import BookApi from "@/api/book"
import BookCover from "@/components/BookCover.vue"
import type { Books, Chapters } from "@/types/global"

const router = useRouter()
const route = useRoute()
const message = useMessage()

const bookId = computed(() => Number(route.query.id) || 0)

const book = ref<Books | null>(null)
const chapterList = ref<Chapters[]>([])
const total = ref(0)
const page = ref(1)
const limit = ref(50)
const loadingChapters = ref(false)

const goBack = () => router.back()

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
  const res = await BookApi.detail(bookId.value)
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
    const res = await BookApi.chapters(bookId.value, page.value, limit.value)
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
})
</script>

<style lang="scss" scoped>
.book-detail {
  position: relative;
  height: 93%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  padding: 20px 24px;
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

.page-scrollbar {
  flex: 1;
  min-height: 0;

  :deep(.n-scrollbar-container) {
    height: 100%;
  }

  :deep(.n-scrollbar-content) {
    min-height: 100%;
  }
}

.detail-header {
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

.intro-scrollbar {
  height: 120px;

  :deep(.n-scrollbar-container) {
    height: 100%;
  }

  :deep(.n-scrollbar-content) {
    padding-right: 8px;
  }
}

.intro-content {
  line-height: 1.6;
  font-size: 13px;
  color: var(--color-text-secondary);
  word-break: break-all;
  white-space: pre-wrap;
}

.detail-divider {
  margin: 16px 0;
}

.chapter-section {
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.chapter-header {
  flex-shrink: 0;
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  margin-bottom: 8px;
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
  max-height: 420px;

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
  width: 100%;
  padding: 8px 10px;
  border-radius: var(--radius-sm);
  transition: background-color 0.2s ease;

  &:hover {
    background-color: var(--color-surface-hover);
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
