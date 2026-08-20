<template>
  <div class="home">
    <header class="home-header">
      <n-input v-model:value="searchValue" class="search-input" round clearable placeholder="搜索书籍">
        <template #prefix>
          <n-icon :component="BookSearch24Regular" />
        </template>
      </n-input>

      <button class="setting-btn" title="设置" aria-label="设置" @click="goSettingView">
        <n-icon :size="20" :component="SettingOutlined" />
      </button>
      <button class="setting-btn" title="阅读" aria-label="阅读" @click="goReaderView">
        <n-icon :size="20" :component="SettingOutlined" />
      </button>
    </header>

    <section class="home-body">
      <div class="section-title">
        <span class="section-name">我的书籍</span>
        <span class="section-count">{{ bookList.length }} 本</span>
      </div>

      <n-scrollbar class="book-scrollbar">
        <n-empty v-if="bookList.length === 0" class="empty" description="还没有导入书籍哦">
          <template #extra>
            <n-button size="small" type="primary" round @click="importBook">立即导入</n-button>
          </template>
        </n-empty>

        <div v-else class="book-grid">
          <div
            v-for="book in bookList"
            :key="book.id"
            class="book-card"
            @click="goDetail(book)"
          >
            <div class="book-cover">
              <img
                v-if="resolveCover(book.cover)"
                :src="resolveCover(book.cover)"
                :alt="book.title"
              />
              <div
                v-else
                class="cover-fallback"
                :style="{ background: coverGradient(book.title) }"
              >
                <span class="cover-spine"></span>
                <div class="cover-meta">
                  <span class="cover-title">{{ book.title }}</span>
                  <span class="cover-author">{{ book.author || "佚名" }}</span>
                </div>
              </div>
            </div>

            <div class="book-info">
              <div class="book-title" :title="book.title">{{ book.title }}</div>
              <div class="book-author">{{ book.author || "佚名" }}</div>
            </div>
          </div>
        </div>
      </n-scrollbar>
    </section>
  </div>
</template>

<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog"
import { convertFileSrc } from "@tauri-apps/api/core"
import { BookSearch24Regular } from "@vicons/fluent"
import { SettingOutlined } from "@vicons/antd"
import BookApi from "@/api/book"
import type { Books } from "@/types/global"

const router = useRouter()
const message = useMessage()

const bookList = ref<Books[]>([])
const searchValue = ref("")

const coverGradients = [
  "linear-gradient(135deg, #667eea, #764ba2)",
  "linear-gradient(135deg, #f093fb, #f5576c)",
  "linear-gradient(135deg, #4facfe, #00f2fe)",
  "linear-gradient(135deg, #43e97b, #38f9d7)",
  "linear-gradient(135deg, #fa709a, #fee140)",
  "linear-gradient(135deg, #30cfd0, #330867)",
  "linear-gradient(135deg, #ff9a9e, #fecfef)",
  "linear-gradient(135deg, #a18cd1, #fbc2eb)",
]

const coverGradient = (title: string) => {
  let hash = 0
  for (let i = 0; i < title.length; i++) {
    hash = (hash * 31 + title.charCodeAt(i)) >>> 0
  }
  return coverGradients[hash % coverGradients.length]
}

const resolveCover = (cover: string) => {
  if (!cover) return ""
  if (/^(https?:|data:|blob:|asset:)/.test(cover)) return cover
  return convertFileSrc(cover)
}

const getBookList = async () => {
  const res = await BookApi.list(searchValue.value)
  if (res.code === 0) {
    bookList.value = res.data
  } else {
    message.error(res.msg)
  }
}

const goSettingView = () => {
  router.push({ name: "Setting" })
}

const goReaderView = () => {
  router.push({ name: "Reader" })
}

const goDetail = (book: Books) => {
  router.push({ name: "BookDetail", query: { id: book.id } })
}

const importBook = async () => {
  const filePath = await open({
    multiple: false,
    filters: [{ name: "TXT", extensions: ["txt", "epub"] }]
  })
  console.log(filePath)
  if (filePath) {
    const res = await BookApi.import(filePath)
    if (res.code === 0) {
      message.success("导入成功")
      getBookList()
    } else {
      message.error(res.msg)
    }
  }
}

onMounted(() => {
  getBookList()
})
</script>

<style lang="scss" scoped>
.home {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.home-header {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  margin: 20px 24px 12px;
}

.search-input {
  flex: 1;
  margin-right: 16px;

  :deep(.n-input) {
    background-color: var(--color-surface);
    border-color: var(--color-border);
  }

  :deep(.n-input:hover) {
    border-color: var(--color-border-hover);
  }

  :deep(.n-input.n-input--focus) {
    border-color: var(--color-accent);
    box-shadow: var(--shadow-focus);
  }
}

.setting-btn {
  flex-shrink: 0;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: var(--radius-sm);
  background-color: var(--color-surface);
  color: var(--color-text-secondary);
  box-shadow: var(--shadow-card);
  cursor: pointer;
  transition: color 0.2s ease, box-shadow 0.2s ease, background-color 0.2s ease;

  &:hover {
    color: var(--color-accent);
    background-color: var(--color-surface-hover);
    box-shadow: var(--shadow-card-hover);
  }
}

.home-body {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  margin: 0 24px 20px;
}

.section-title {
  flex-shrink: 0;
  display: flex;
  align-items: baseline;
  margin-bottom: 12px;
}

.section-name {
  font-size: 15px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin-right: 8px;
}

.section-count {
  font-size: 12px;
  color: var(--color-text-secondary);
}

.book-scrollbar {
  flex: 1;
  min-height: 0;

  :deep(.n-scrollbar-container) {
    height: 100%;
  }

  :deep(.n-scrollbar-content) {
    min-height: 100%;
  }
}

.empty {
  min-height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.book-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 20px;
  padding-bottom: 8px;
}

.book-card {
  cursor: pointer;
  display: flex;
  flex-direction: column;
}

.book-cover {
  position: relative;
  aspect-ratio: 2 / 3;
  border-radius: var(--radius-sm);
  overflow: hidden;
  box-shadow: var(--shadow-card);
  transition: transform 0.2s ease, box-shadow 0.2s ease;

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }
}

.book-card:hover .book-cover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-card-hover);
}

.cover-fallback {
  position: relative;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 16px 12px;
  color: #fff;
  text-align: center;
}

.cover-spine {
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 8px;
  background: rgba(0, 0, 0, 0.18);
}

.cover-meta {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}

.cover-title {
  font-size: 14px;
  font-weight: 700;
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-shadow: 0 1px 3px rgba(0, 0, 0, 0.25);
}

.cover-author {
  font-size: 12px;
  opacity: 0.85;
  display: -webkit-box;
  -webkit-line-clamp: 1;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.book-info {
  margin-top: 10px;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.book-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.book-author {
  font-size: 12px;
  color: var(--color-text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
