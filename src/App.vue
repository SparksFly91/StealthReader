<template>
  <n-config-provider :locale="zhCN" :data-locale="dateZhCN">
    <n-message-provider>
      <n-notification-provider>
        <n-dialog-provider>
          <GlobalLoading />
          <RouterView />
        </n-dialog-provider>
      </n-notification-provider>
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { zhCN, dateZhCN } from "naive-ui"
import GlobalLoading from "@/components/GlobalLoading.vue"
import { checkAndUpdate } from "@/utils/updater"

onMounted(() => {
  checkAndUpdate()
})
</script>

<style scoped>

</style>
