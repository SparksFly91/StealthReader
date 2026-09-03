# CODEBUDDY.md

## 项目概述

**StealthReader（幽灵阅读器）** — 安静、本地优先的桌面小说阅读器。轻量、纯本地、无账号、无广告。

- 技术栈：Tauri 2（Rust 后端）+ Vue 3 + TypeScript + Vite + Naive UI + Pinia
- 包管理：pnpm（`pnpm-workspace.yaml`）
- 桌面壳：`src-tauri/`（Rust：sqlx + SQLite、chardetng 编码检测、regex 分章）
- 当前版本：v0.3.2（来自 `package.json`，与 Cargo.toml / tauri.conf.json 同步）

## 常用命令

| 命令 | 说明 |
| --- | --- |
| `pnpm dev` | 仅启动 Vite 前端（http://localhost:1420，strictPort） |
| `pnpm build` | `vue-tsc --noEmit` 类型检查 + Vite 构建 |
| `pnpm tauri dev` | 开发模式启动桌面应用 |
| `pnpm tauri build` | 构建安装包（含 updater 产物） |
| `pnpm preview` | 预览构建产物 |

## 关键约定

### 前后端通信（务必遵守）
- 所有 Rust 命令通过 `src/api/book.ts` 用 `invoke` 封装调用，统一返回 `ApiResponse<T>`（`{ success, code, msg, data }`）。
- **参数命名**：前端传 camelCase（如 `bookId`），Rust 端是 snake_case（`book_id`），Tauri 自动转换。
- **类型双写**：前端 `src/types/global.ts` 与 Rust `src-tauri/src/models.rs` 必须手动保持同步，改一处必须改另一处。

### 新增 Rust 命令
- 在 `src-tauri/src/commands/book.rs`（或新命令文件）定义函数，加 `#[auto_collect_command]`（来自 `tauri-helper` 宏），会自动注册，**无需手改 invoke_handler 列表**。

### 数据库
- 无独立 migrations，建表 SQL 内联于 `src-tauri/src/services/db.rs` 的 `init_pool` 中。库文件 `reader.db` 位于 app_data_dir。
- 三张表：`books`、`chapters`（索引 `idx_chapters_book(book_id, number)`）、`reading_progress`（UNIQUE(book_id, chapter_id)）。
- 改 schema 直接编辑 `db.rs` 内联 SQL；已有旧库兼容迁移逻辑也在这里，改动需注意升级路径。

### 路由
- 懒加载，history 模式。`/` → MainLayout（重定向 `/home`）。
- `/setting` 重定向 `/setting/appearance`，子页：appearance / reader-setting / shortcut / about。
- `/reader` 为独立全屏阅读器路由，**不在 MainLayout 内**。

### 状态与持久化
- Pinia + `pinia-plugin-persistedstate`，设置存于 `src/stores/setting.ts`，跨会话持久化。
- 新增可持久化 store 时同样用该插件，无需手写 localStorage。

### 窗口特性
- 无边框透明窗口（`decorations: false`、`transparent: true`、`shadow: false`、`resizable: false`）。
- 自定义标题栏分平台组件：`src/layout/components/WindowsTitleBar.vue`、`MacTitleBar.vue`，改动标题栏需同时考虑两平台。

### 更新机制
- 前端更新逻辑封装在 `src/utils/updater.ts`，配套 `src/components/UpdateModal.vue`。
- updater endpoint 指向 GitHub Releases latest.json（含 pubkey 签名），发布 v\* tag 会触发 `.github/workflows/release.yml`（macOS + Windows 矩阵构建）。

### 自动导入
- `unplugin-auto-import` 自动导入 Vue / vue-router / naive-ui 的 `useDialog`、`useMessage` 等 API；`unplugin-vue-components` + NaiveUiResolver 自动注册组件。
- 类型声明生成在 `src/types/auto-imports.d.ts` 与 `src/types/components.d.ts`，**不要手改**。

## 代码风格

- 路径别名 `@` → `src`。
- 图标统一用 `@vicons`（antd / carbon / fa / fluent / ionicons / material / tabler）。
- 日期时间用 `dayjs`；富文本预览用 `markdown-it`。
- 无 eslint/prettier 配置；类型检查靠 `vue-tsc --noEmit`（build 内置），提交前保证 `pnpm build` 通过。
- 注释语言与项目保持一致，使用简体中文。

## 注意事项

- **仅支持本地文件导入**，不内置在线书源或下载功能（详见 README 免责声明）。
- 修改版本号时需同步：`package.json`、`src-tauri/Cargo.toml`、`src-tauri/tauri.conf.json`、`docs/CHANGELOG.md`。
- `pnpm dev` 端口固定 1420（strictPort），被占用会直接失败。
