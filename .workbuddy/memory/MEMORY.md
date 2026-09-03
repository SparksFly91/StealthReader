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
