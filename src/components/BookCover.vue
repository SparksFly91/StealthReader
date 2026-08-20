<template>
  <div class="book-cover">
    <img :src="resolvedSrc" :alt="title" />
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { convertFileSrc } from "@tauri-apps/api/core"
import defaultCover from "@/assets/cover.jpg"

const props = defineProps<{
  title: string
  author?: string
  cover?: string
}>()

const resolvedSrc = computed(() => {
  const cover = props.cover
  if (cover) {
    if (/^(https?:|data:|blob:|asset:)/.test(cover)) return cover
    return convertFileSrc(cover)
  }
  return defaultCover
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
</style>
