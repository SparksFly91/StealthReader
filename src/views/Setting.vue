<template>
  <n-space>
    <n-icon :component="MdArrowBack" @click="goBack" />
    <p><strong>设置页面</strong></p>
  </n-space>
</template>

<script setup lang="ts">
import { MdArrowBack } from "@vicons/ionicons4"

const router = useRouter()

const goBack = () => {
    router.back()
}
</script>

<style scoped></style>
