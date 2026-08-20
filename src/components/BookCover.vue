<template>
  <div class="book-cover">
    <img v-if="resolvedSrc" :src="resolvedSrc" :alt="title" />
    <div v-else class="cover-fallback" :style="{ background: coverGradient(title) }">
      <span class="cover-spine"></span>
      <div class="cover-meta">
        <p class="cover-title">{{ title }}</p>
        <span class="cover-author">{{ author || "佚名" }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { convertFileSrc } from "@tauri-apps/api/core"

const props = defineProps<{
  title: string
  author?: string
  cover?: string
}>()

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

const resolvedSrc = computed(() => {
  const cover = props.cover
  if (!cover) return ""
  if (/^(https?:|data:|blob:|asset:)/.test(cover)) return cover
  return convertFileSrc(cover)
})
</script>

<style lang="scss" scoped>
.book-cover {
  position: relative;
  width: 100%;
  height: 100%;
  overflow: hidden;
  border-radius: var(--radius-sm);

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }
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
</style>
