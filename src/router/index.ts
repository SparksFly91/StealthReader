import { createRouter, createWebHistory } from "vue-router"
import MainLayout from "@/layout/MainLayout.vue"
import Home from "@/views/Home.vue"
import BookDetail from "@/views/BookDetail.vue"
import Setting from "@/views/Setting.vue"
import Appearance from "@/views/setting/Appearance.vue"
import ReaderSetting from "@/views/setting/ReaderSetting.vue"
import Shortcut from "@/views/setting/Shortcut.vue"
import About from "@/views/setting/About.vue"
import Reader from "@/views/Reader.vue"

const routes = [
  {
    path: "/",
    name: "MainLayout",
    component: MainLayout,
    redirect: "/home",
    children: [
      {
        path: "/home",
        name: "Home",
        component: Home,
        meta: {
          isCache: true,
          title: "首页",
        },
      },
      {
        path: "/book-detail",
        name: "BookDetail",
        component: BookDetail,
        meta: {
          isCache: true,
          title: "书籍详情",
        },
      },

      {
        path: "/setting",
        name: "Setting",
        component: Setting,
        redirect: "/setting/appearance",
        meta: {
          isCache: true,
          title: "设置",
        },
        children: [
          {
            path: "/setting/appearance",
            name: "Appearance",
            component: Appearance,
            meta: {
              isCache: true,
              title: "外观设置",
            },
          },
          {
            path: "/setting/reader-setting",
            name: "ReaderSetting",
            component: ReaderSetting,
            meta: {
              isCache: true,
              title: "阅读设置",
            },
          },
          {
            path: "/setting/shortcut",
            name: "Shortcut",
            component: Shortcut,
            meta: {
              isCache: true,
              title: "快捷键",
            },
          },
          {
            path: "/setting/about",
            name: "About",
            component: About,
            meta: {
              isCache: true,
              title: "关于",
            },
          },
        ],
      },
    ],
  },
  {
    path: "/reader",
    name: "Reader",
    component: Reader,
    meta: {
      isCache: true,
      title: "阅读器",
    },
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
