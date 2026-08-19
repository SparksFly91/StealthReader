import { createRouter, createWebHistory } from "vue-router"

const routes = [
  {
    path: "/",
    name: "MainLayout",
    component: () => import("@/layout/MainLayout.vue"),
    redirect: "/home",
    children: [
      {
        path: "/home",
        name: "Home",
        component: () => import("@/views/Home.vue"),
        meta: {
          isCache: true,
          title: "首页",
        },
      },
      {
        path: "/book-detail",
        name: "BookDetail",
        component: () => import("@/views/BookDetail.vue"),
        meta: {
          isCache: true,
          title: "书籍详情",
        },
      },

      {
        path: "/setting",
        name: "Setting",
        component: () => import("@/views/Setting.vue"),
        meta: {
          isCache: true,
          title: "设置",
        },
        children: [
          {
            path: "/setting/appearance",
            name: "Appearance",
            component: () => import("@/views/setting/Appearance.vue"),
            meta: {
              isCache: true,
              title: "外观设置",
            },
          },
        ],
      },
    ],
  },
  {
    path: "/reader",
    name: "Reader",
    component: () => import("@/views/Reader.vue"),
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
