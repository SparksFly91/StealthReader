<template>
  <div class="home">
    <header class="home-header">
      <n-input v-model:value="searchValue" class="search-input" round clearable placeholder="搜索书籍" @click.enter="getBookList">
        <template #prefix>
          <n-icon :component="BookSearch24Regular" />
        </template>
      </n-input>

      <button class="setting-btn" title="设置" aria-label="设置" @click="goSettingView">
        <n-icon :size="20" :component="SettingOutlined" />
      </button>
      <button class="setting-btn" title="导入" aria-label="导入" @click="importBook">
        <n-icon :size="20" :component="ImportOutlined" />
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
            @contextmenu.prevent="onContextMenu($event, book)"
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

    <n-dropdown
      trigger="manual"
      :show="showDropdown"
      :x="dropdownX"
      :y="dropdownY"
      :options="dropdownOptions"
      @select="handleDropdownSelect"
      @clickoutside="showDropdown = false"
    />

    <n-modal
      v-model:show="showEditModal"
      preset="card"
      title="编辑书籍"
      :bordered="false"
      style="width: 480px"
    >
      <n-form
        ref="formRef"
        :model="editForm"
        :rules="rules"
        label-placement="left"
        label-width="64"
        require-mark-placement="left"
      >
        <n-form-item label="书名" path="title">
          <n-input v-model:value="editForm.title" placeholder="请输入书名" />
        </n-form-item>
        <n-form-item label="作者" path="author">
          <n-input v-model:value="editForm.author" placeholder="请输入作者" />
        </n-form-item>
        <n-form-item label="简介" path="introduction">
          <n-input
            v-model:value="editForm.introduction"
            type="textarea"
            placeholder="请输入简介"
            :autosize="{ minRows: 3, maxRows: 6 }"
          />
        </n-form-item>
      </n-form>
      <template #footer>
        <n-space justify="end">
          <n-button @click="showEditModal = false">取消</n-button>
          <n-button type="primary" :loading="saving" @click="submitEdit">保存</n-button>
        </n-space>
      </template>
    </n-modal>

    <n-modal
      v-model:show="showDeleteModal"
      preset="card"
      title="删除书籍"
      :bordered="false"
      style="width: 400px"
    >
      <div class="delete-tip">
        确定要删除《{{ deleteBook?.title }}》吗？删除后不可恢复。
      </div>
      <template #footer>
        <n-space justify="end">
          <n-button @click="showDeleteModal = false">取消</n-button>
          <n-button type="error" :loading="deleting" @click="doDelete">删除</n-button>
        </n-space>
      </template>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { h, type Component } from "vue"
import { open } from "@tauri-apps/plugin-dialog"
import { BookSearch24Regular } from "@vicons/fluent"
import {
  SettingOutlined,
  ImportOutlined,
  EditOutlined,
  DeleteOutlined,
} from "@vicons/antd"
import { NIcon } from "naive-ui"
import BookApi from "@/api/book"
import BookCover from "@/components/BookCover.vue"
import { useLoading } from "@/hooks/useLoading"
import type { Books, BookSaveParams } from "@/types/global"

const router = useRouter()
const message = useMessage()
const { withLoading } = useLoading()

const bookList = ref<Books[]>([])
const searchValue = ref("")

const showDeleteModal = ref(false)
const deleting = ref(false)
const deleteBook = ref<Books | null>(null)

const showDropdown = ref(false)
const dropdownX = ref(0)
const dropdownY = ref(0)
const contextBook = ref<Books | null>(null)

const renderDropdownIcon = (icon: Component) => () => h(NIcon, null, { default: () => h(icon) })

const dropdownOptions = [
  { label: "编辑", key: "edit", icon: renderDropdownIcon(EditOutlined) },
  { label: "删除", key: "delete", icon: renderDropdownIcon(DeleteOutlined) },
]

const showEditModal = ref(false)
const saving = ref(false)
const formRef = ref<any>()

const editForm = reactive<BookSaveParams>({
  id: 0,
  title: "",
  author: "",
  cover: "",
  introduction: "",
})

const rules = {
  title: { required: true, message: "请输入书名", trigger: ["blur", "input"] },
}

const getBookList = async () => {
  const res = await withLoading(() => BookApi.list(searchValue.value), "加载中...")
  if (res.code === 0) {
    bookList.value = res.data
  } else {
    message.error(res.msg)
  }
}

const goSettingView = () => {
  router.push({ name: "Setting" })
}

const goDetail = (book: Books) => {
  router.push({ name: "BookDetail", query: { id: book.id } })
}

const onContextMenu = (e: MouseEvent, book: Books) => {
  contextBook.value = book
  dropdownX.value = e.clientX
  dropdownY.value = e.clientY
  showDropdown.value = true
}

const handleDropdownSelect = (key: string | number) => {
  showDropdown.value = false
  if (key === "edit") {
    openEditModal(contextBook.value)
  } else if (key === "delete") {
    confirmDelete(contextBook.value)
  }
}

const openEditModal = (book: Books | null) => {
  if (!book) return
  editForm.id = book.id
  editForm.title = book.title
  editForm.author = book.author
  editForm.cover = book.cover
  editForm.introduction = book.introduction || ""
  showEditModal.value = true
}

const submitEdit = async () => {
  try {
    await formRef.value?.validate()
  } catch {
    return
  }
  saving.value = true
  try {
    const res = await BookApi.edit({ ...editForm })
    if (res.code === 0) {
      message.success("保存成功")
      showEditModal.value = false
      getBookList()
    } else {
      message.error(res.msg)
    }
  } finally {
    saving.value = false
  }
}

const confirmDelete = (book: Books | null) => {
  if (!book) return
  deleteBook.value = book
  showDeleteModal.value = true
}

const doDelete = async () => {
  const book = deleteBook.value
  if (!book || deleting.value) return
  deleting.value = true
  try {
    const res = await withLoading(() => BookApi.del(book.id), "删除中...")
    if (res.code === 0) {
      message.success("删除成功")
      showDeleteModal.value = false
      getBookList()
    } else {
      message.error(res.msg)
    }
  } catch (e) {
    message.error("删除失败，请重试")
  } finally {
    deleting.value = false
  }
}

const importBook = async () => {
  const filePath = await open({
    multiple: false,
    filters: [{ name: "TXT", extensions: ["txt", "epub"] }]
  })
  if (filePath) {
    const res = await withLoading(() => BookApi.import(filePath), "导入中...")
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
  margin-top: 5px;
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

.delete-tip {
  font-size: 14px;
  line-height: 1.6;
  color: var(--color-text-primary);
  word-break: break-all;
}
</style>
