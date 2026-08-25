<template>
  <n-config-provider :locale="zhCN" :data-locale="dateZhCN" :theme="isDark ? darkTheme : null">
    <n-message-provider>
      <n-notification-provider>
        <n-dialog-provider>
          <n-modal-provider>
            <GlobalLoading />
            <UpdateModal />
            <RouterView />
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
