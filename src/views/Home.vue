<template>
  <n-space vertical size="large">
    <n-layout>
      <n-layout-header class="header">
        <n-input round placeholder="搜索">
          <template #suffix>
            <n-icon :component="BookSearch24Regular" />
          </template>
        </n-input>
        <n-icon :component="SettingOutlined" @click="goSettingView" />
        <!-- <n-icon :component="BookReader" @click="goReaderView" /> -->
      </n-layout-header>
      <n-layout-content class="content">
        <div>书籍列表</div>
        <n-empty v-if="bookList.length == 0" description="还没有导入书籍哦">
          <template #extra>
            <n-button size="small"> 立即导入 </n-button>
          </template>
        </n-empty>
      </n-layout-content>
    </n-layout>
  </n-space>
</template>

<script setup lang="ts">
import { BookSearch24Regular } from "@vicons/fluent"
import { SettingOutlined } from "@vicons/antd"
import { BookReader } from "@vicons/fa"

const router = useRouter()

const bookList = ref([])

const goSettingView = () => {
  router.push({ name: "Setting" })
}

const goReaderView = () => {
  router.push({ name: "Reader" })
}
</script>

<style lang="scss" scoped></style>
