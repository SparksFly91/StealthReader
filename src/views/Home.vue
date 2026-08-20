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
      <button class="setting-btn" title="导入" aria-label="导入" @click="importBook">
        <n-icon :size="20" :component="CloudUploadOutlined" />
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
            <div class="book-cover-wrap">
              <BookCover :title="book.title" :author="book.author" :cover="book.cover" />
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
import { BookSearch24Regular } from "@vicons/fluent"
import { SettingOutlined, CloudUploadOutlined } from "@vicons/antd"
import BookApi from "@/api/book"
import BookCover from "@/components/BookCover.vue"
import type { Books } from "@/types/global"

const router = useRouter()
const message = useMessage()

const bookList = ref<Books[]>([])
const searchValue = ref("")

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

.book-cover-wrap {
  aspect-ratio: 2 / 3;
  border-radius: var(--radius-sm);
  overflow: hidden;
  box-shadow: var(--shadow-card);
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.book-card:hover .book-cover-wrap {
  transform: translateY(-4px);
  box-shadow: var(--shadow-card-hover);
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
