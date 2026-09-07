# StealthReader 项目长期笔记

## 构建环境（Windows + Git Bash，本机特有）

### pnpm 命令
Git Bash 下 `pnpm` shim 会误解析路径（`Cannot find module 'F:\c\Users\...'`）。改为用 node 直跑：
```bash
node "C:/Users/OrBit-0297/AppData/Local/pnpm/global/v11/<hash>/node_modules/pnpm/bin/pnpm.mjs" <add|install|...>
```

### cargo check / build 链接环境
Git Bash 下 GNU coreutils 的 `link` 会遮蔽 MSVC `link.exe`，需前置 MSVC bin 并导出库/头文件搜索路径：
```bash
export PATH="/d/VisualStudio/BuildTools/VC/Tools/MSVC/14.51.36231/bin/Hostx64/x64:$PATH"
export LIB='D:\VisualStudio\BuildTools\VC\Tools\MSVC\14.51.36231\lib\x64;C:\Program Files (x86)\Windows Kits\10\Lib\10.0.26100.0\um\x64;C:\Program Files (x86)\Windows Kits\10\Lib\10.0.26100.0\ucrt\x64'
export INCLUDE='D:\VisualStudio\BuildTools\VC\Tools\MSVC\14.51.36231\include;C:\Program Files (x86)\Windows Kits\10\Include\10.0.26100.0\ucrt;C:\Program Files (x86)\Windows Kits\10\Include\10.0.26100.0\um;C:\Program Files (x86)\Windows Kits\10\Include\10.0.26100.0\shared'
```

## 意见反馈功能要点

- SMTP 配置运行时读取自 `app_config_dir()/feedback.config.json`，首次运行自动生成模板（`feedback.config.example.json` 经 include_str! 嵌入）；授权码不随代码提交、不打进二进制。
- 版本号实际只同步 package.json（tauri.conf.json 引用 `../package.json`）+ CHANGELOG；`Cargo.toml` 一直保持 0.1.0 未同步（与 CODEBUDDY.md 描述有出入）。

## CHANGELOG 与 git tag 容易错位（曾踩坑）

- 历史上 CHANGELOG 顶部出现「v0.3.3 (2026-09-03)」但 git 里**不存在 v0.3.3 tag**，且 v0.3.3 内容（意见反馈）其实是 v0.4.0 之后才提交的；同时 v0.4.0 tag 自带的 CHANGELOG 段落（在 `git show v0.4.0:docs/CHANGELOG.md` 可见，包含右键调色/首行缩进/更新弹窗重构/圆角修复）被后来提交覆盖丢失。
- 写新版本 CHANGELOG 时务必先 `git log <last_tag>..HEAD --no-merges` + `git show <last_tag>:docs/CHANGELOG.md` 双向校验，避免再次错位。
- 写完后建议 `git tag v0.x.y` 再 commit，避免版本号与 tag 不同步。
