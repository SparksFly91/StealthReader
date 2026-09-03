<template>
  <div class="feedback">
    <h2 class="page-title">意见反馈</h2>
    <p class="page-desc">反馈内容将直接发送至开发者邮箱，仅用于问题处理</p>

    <div class="feedback-form">
      <div class="form-item">
        <label class="form-label">你的邮箱</label>
        <n-input v-model:value="email" clearable placeholder="用于接收回复" />
      </div>

      <div class="form-item">
        <label class="form-label">反馈内容</label>
        <div class="editor-card">
          <div class="editor-toolbar">
            <button
              v-for="btn in toolbar"
              :key="btn.title"
              class="toolbar-btn"
              type="button"
              :title="btn.title"
              @click="btn.action"
            >
              <n-icon :component="btn.icon" :size="16" />
            </button>
          </div>
          <EditorContent :editor="editor" class="editor-content" />
        </div>
        <div class="editor-hint">支持加粗、列表等格式，可直接粘贴截图（单张不超过 2MB）</div>
      </div>

      <div class="form-actions">
        <n-button type="primary" :loading="sending" round @click="onSubmit">发送反馈</n-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useEditor, EditorContent } from "@tiptap/vue-3"
import StarterKit from "@tiptap/starter-kit"
import Image from "@tiptap/extension-image"
import DOMPurify from "dompurify"
import {
  BoldOutlined,
  ItalicOutlined,
  StrikethroughOutlined,
  CodeOutlined,
  UnorderedListOutlined,
  OrderedListOutlined,
  FontSizeOutlined,
} from "@vicons/antd"
import FeedbackApi from "@/api/feedback"
import type { FeedbackImage } from "@/types/global"

const message = useMessage()
const email = ref("")
const sending = ref(false)

const editor = useEditor({
  content: "",
  extensions: [StarterKit, Image.configure({ allowBase64: true })],
  editorProps: {
    handlePaste: (_view, event) => {
      const items = event.clipboardData?.items
      if (!items) return false
      for (const item of items) {
        if (item.type.startsWith("image/")) {
          const file = item.getAsFile()
          if (file) insertImage(file)
          return true
        }
      }
      return false
    },
    handleDrop: (_view, event) => {
      const files = event.dataTransfer?.files
      if (!files) return false
      let handled = false
      for (const file of files) {
        if (file.type.startsWith("image/")) {
          insertImage(file)
          handled = true
        }
      }
      return handled
    },
  },
})

const toolbar = [
  { title: "加粗", icon: BoldOutlined, action: () => editor.value?.chain().focus().toggleBold().run() },
  { title: "斜体", icon: ItalicOutlined, action: () => editor.value?.chain().focus().toggleItalic().run() },
  { title: "删除线", icon: StrikethroughOutlined, action: () => editor.value?.chain().focus().toggleStrike().run() },
  { title: "行内代码", icon: CodeOutlined, action: () => editor.value?.chain().focus().toggleCode().run() },
  { title: "无序列表", icon: UnorderedListOutlined, action: () => editor.value?.chain().focus().toggleBulletList().run() },
  { title: "有序列表", icon: OrderedListOutlined, action: () => editor.value?.chain().focus().toggleOrderedList().run() },
  { title: "标题", icon: FontSizeOutlined, action: () => editor.value?.chain().focus().toggleHeading({ level: 2 }).run() },
]

function insertImage(file: File) {
  if (file.size > 2 * 1024 * 1024) {
    message.warning("图片不能超过 2MB")
    return
  }
  const reader = new FileReader()
  reader.onload = () => {
    editor.value?.chain().focus().setImage({ src: reader.result as string }).run()
  }
  reader.readAsDataURL(file)
}

function isValidEmail(val: string): boolean {
  const parts = val.split("@")
  if (parts.length !== 2) return false
  const [local, domain] = parts
  return !!local && domain.includes(".") && !val.includes(" ")
}

/** 提取 HTML 中的 base64 图片，替换为 cid 引用，返回内联图片列表 */
function extractImages(html: string): { html: string; images: FeedbackImage[] } {
  const images: FeedbackImage[] = []
  let index = 0
  const newHtml = html.replace(
    /<img[^>]+src="data:(image\/[a-zA-Z+.-]+);base64,([^"]+)"[^>]*>/g,
    (full, mime: string, b64: string) => {
      const cid = `img_${index++}@feedback`
      images.push({ cid, mime, base64: b64 })
      return full.replace(`src="data:${mime};base64,${b64}"`, `src="cid:${cid}"`)
    },
  )
  return { html: newHtml, images }
}

function htmlToText(html: string): string {
  const div = document.createElement("div")
  div.innerHTML = html
  return (div.textContent || "").trim()
}

const onSubmit = async () => {
  const val = email.value.trim()
  if (!val) {
    message.warning("请填写邮箱")
    return
  }
  if (!isValidEmail(val)) {
    message.warning("邮箱格式不正确")
    return
  }

  const rawHtml = editor.value?.getHTML() || ""
  const cleanHtml = DOMPurify.sanitize(rawHtml)
  if (!cleanHtml || cleanHtml === "<p></p>") {
    message.warning("请填写反馈内容")
    return
  }

  const { html, images } = extractImages(cleanHtml)
  if (images.length > 5) {
    message.warning("图片最多 5 张")
    return
  }
  const text = htmlToText(cleanHtml)

  sending.value = true
  try {
    const res = await FeedbackApi.send({ email: val, html, text, images })
    if (res.code === 0) {
      message.success("反馈已发送，感谢你的建议！")
      editor.value?.commands.clearContent()
      email.value = ""
    } else {
      message.error(res.msg)
    }
  } catch {
    message.error("发送失败，请检查网络后重试")
  } finally {
    sending.value = false
  }
}

onBeforeUnmount(() => {
  editor.value?.destroy()
})
</script>

<style lang="scss" scoped>
.feedback {
  padding: 20px 24px;
}

.page-title {
  margin: 0 0 2px;
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.page-desc {
  margin: 0 0 18px;
  font-size: 13px;
  color: var(--color-text-secondary);
}

.feedback-form {
  max-width: 640px;
}

.form-item {
  margin-bottom: 18px;
}

.form-label {
  display: block;
  margin-bottom: 8px;
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.editor-card {
  overflow: hidden;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  background-color: var(--color-surface);
}

.editor-toolbar {
  display: flex;
  gap: 2px;
  padding: 6px 8px;
  border-bottom: 1px solid var(--color-border);
  background-color: var(--color-window-bg);
}

.toolbar-btn {
  width: 30px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 6px;
  background-color: transparent;
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: color 0.2s ease, background-color 0.2s ease;

  &:hover {
    color: var(--color-accent);
    background-color: var(--color-surface-hover);
  }
}

.editor-content {
  :deep(.ProseMirror) {
    min-height: 200px;
    max-height: 360px;
    overflow-y: auto;
    padding: 12px 14px;
    outline: none;
    font-size: 14px;
    line-height: 1.7;
    color: var(--color-text-primary);

    img {
      max-width: 100%;
      border-radius: 6px;
    }

    p {
      margin: 0 0 8px;
    }

    h2 {
      font-size: 16px;
      font-weight: 600;
      margin: 12px 0 8px;
    }

    ul,
    ol {
      padding-left: 22px;
      margin: 0 0 8px;
    }

    blockquote {
      margin: 0 0 8px;
      padding-left: 12px;
      border-left: 3px solid var(--color-border);
      color: var(--color-text-secondary);
    }

    code {
      padding: 2px 6px;
      border-radius: 4px;
      background-color: var(--color-window-bg);
      font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    }

    pre {
      padding: 12px;
      border-radius: 6px;
      background-color: var(--color-window-bg);
      overflow-x: auto;
      margin: 0 0 8px;

      code {
        padding: 0;
        background-color: transparent;
      }
    }
  }
}

.editor-hint {
  margin-top: 6px;
  font-size: 12px;
  color: var(--color-text-secondary);
}

.form-actions {
  margin-top: 4px;
}
</style>
