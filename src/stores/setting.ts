import { defineStore } from "pinia"

export const useSettingStore = defineStore("setting", {
  state: () => ({
    appearance: {
        theme: "light",
        showShadow: false,
    },
    reader: {
        fontSize: 12,
        showShadow: false,
        fontColor: "#000000",
        backgroundColor: "#ffffff",
        windowWidth: 300,
        windowHeight: 400,
    }
  }),

  persist: true,
})
