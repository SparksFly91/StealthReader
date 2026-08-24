# 幽灵阅读器 · Stealth Reader

一款安静、本地优先的桌面小说阅读器。轻量、纯本地、无账号、无广告，专注「好好读一本书」。

## 已实现功能

### 书籍导入与解析
- **本地 TXT 导入**：通过系统文件选择对话框导入，无需任何在线源
- **编码自动检测**：基于 `chardetng` 自动识别 GBK / UTF-8 等常见中文编码
- **自动分章**：正则识别「第 X 章」「Chapter N」「楔子 / 序章 / 前言 / 引子 / 后记 / 尾声 / 番外」等格式；无法识别时整本书作为单章「正文」导入
- **元数据统计**：自动统计章节数与总字数

### 书架管理
- 封面网格展示（本地封面文件 / 默认封面）
- 按书名模糊搜索
- 右键菜单编辑（书名 / 作者 / 简介）与删除（二次确认）
- 空状态引导一键导入

### 书籍详情
- 封面 + 作者 / 章节数 / 总字数 / 导入时间 / 上次阅读时间 / 阅读进度
- 简介悬停浮层（超长简介内可滚动）
- 章节列表分页展示（每页 20 / 50 / 100 可选）

### 分页阅读
- 点击页面左右半屏翻页，或使用键盘（`↑↓` / `PgUp` / `PgDn` / 空格 / `←→`）
- 上一章 / 下一章快速切换，边界提示
- 阅读外观实时生效：字体大小、字体颜色、背景颜色与透明度、窗口大小
- 阅读窗口可拖拽，支持窗口阴影开关

### 设置中心
- **外观**：窗口阴影开关（主题模式选项已预留）
- **阅读设置**：字体大小、字体颜色、背景颜色与透明度、阅读窗口大小、窗口阴影
- **快捷键**：按键说明参考
- **关于**：版本信息

### 窗口与界面
- 自定义标题栏（Windows / macOS）：仅保留关闭与最小化按钮，标题栏区域可拖拽窗口
- 窗口支持透明背景、置顶显示、圆角毛玻璃外观
- 设置通过 Pinia + 持久化插件跨会话保存

## 界面预览

| 书架管理 | 书籍详情 |
| --- | --- |
| ![书架管理](images/1.png) | ![书籍章节详情](images/3.png) |

| 幽灵阅读模式 | 阅读设置 |
| --- | --- |
| ![幽灵阅读模式](images/4.png) | ![阅读设置](images/2.png) |

## 技术栈

| 层 | 技术 |
|----|------|
| 桌面壳 | [Tauri 2](https://tauri.app/) |
| 前端 | [Vue 3](https://vuejs.org/) + TypeScript + [Vite](https://vite.dev/) |
| 组件库 | [Naive UI](https://www.naiveui.com/) + @vicons 图标 |
| 状态 | [Pinia](https://pinia.vuejs.org/) + `pinia-plugin-persistedstate` |
| 后端 | Rust：`sqlx` + SQLite、`chardetng`（编码检测）、`regex`（分章） |
| 插件 | `tauri-plugin-dialog`（文件选择）、`tauri-plugin-store`、`tauri-plugin-opener` |

## 快速开始

### 环境要求

- [Node.js](https://nodejs.org/)（18+）与 [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/) 工具链
- Tauri 各平台系统依赖（参考 [Tauri 官方文档](https://tauri.app/start/prerequisites/)）

### 安装与运行

```bash
pnpm install        # 安装依赖
pnpm tauri dev      # 开发模式启动
pnpm tauri build    # 构建安装包
```

仅调试前端界面：

```bash
pnpm dev            # 启动 Vite 开发服务器（http://localhost:1420）
pnpm build          # 类型检查 + 构建前端产物
```

## 项目结构

```
src/
├── api/            # Tauri invoke 封装（书籍 / 章节）
├── components/     # BookCover、GlobalLoading
├── hooks/          # useLoading
├── layout/         # 主布局与自定义标题栏（Windows / macOS）
├── stores/         # Pinia（setting 持久化）
├── types/          # 全局类型定义
└── views/          # Home / BookDetail / Reader / Setting 等页面
src-tauri/
└── src/
    ├── commands/   # Tauri 命令（book_*、chapter_*）
    ├── services/   # SQLite 初始化、TXT 解析
    └── models/     # 数据模型
```

## 路线图

- 老板键（系统级全局快捷键，一键显隐）
- 系统托盘与伪装图标
- 阅读进度自动保存与续读
- EPUB 解析支持
- 主题切换实际应用（浅色 / 深色 / 跟随系统）
- 幽灵模式（鼠标穿透 + 极小悬浮窗）
- 多设备进度同步（WebDAV / 局域网）
- 语音朗读（TTS）、书摘 / 笔记

## 免责声明

本项目仅支持本地文件导入，不内置任何在线书源或下载功能。请尊重版权，仅使用您拥有合法权利的书籍内容。
