<template>
  <n-config-provider :locale="zhCN" :data-locale="dateZhCN" :theme="isDark ? darkTheme : null">
    <n-message-provider :container-style="messageContainerStyle">
      <n-notification-provider>
        <n-dialog-provider>
          <n-modal-provider>
            <GlobalLoading />
            <UpdateModal />
            <Suspense>
              <RouterView />
            </Suspense>
          </n-modal-provider>
        </n-dialog-provider>
      </n-notification-provider>
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { zhCN, dateZhCN, darkTheme } from "naive-ui"
import GlobalLoading from "@/components/GlobalLoading.vue"
import UpdateModal from "@/components/UpdateModal.vue"
import { checkAndPrompt } from "@/utils/updater"
import { useSettingStore } from "@/stores/setting"

const settingStore = useSettingStore()

// 消息弹窗容器下移：默认 top: 20px 会压在 38px 高的自定义标题栏上
const messageContainerStyle = { top: "56px" }

const media = window.matchMedia("(prefers-color-scheme: dark)")
const systemDark = ref(media.matches)

const themeMode = computed(() => settingStore.appearance.theme)
const isDark = computed(() => {
  return themeMode.value === "dark" || (themeMode.value === "system" && systemDark.value)
})

watchEffect(() => {
  document.documentElement.setAttribute("data-theme", isDark.value ? "dark" : "light")
})

const onSystemThemeChange = (e: MediaQueryListEvent) => {
  systemDark.value = e.matches
}

onMounted(() => {
  checkAndPrompt()
  media.addEventListener("change", onSystemThemeChange)
})

onUnmounted(() => {
  media.removeEventListener("change", onSystemThemeChange)
})
</script>

<style scoped></style>
