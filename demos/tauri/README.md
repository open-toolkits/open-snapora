# open-snapora Tauri v2 + Rust Demo

> 基于 **Tauri v2 + Rust (`xcap`) + Web Overlay** 的高性能桌面截屏与标注演示程序。

---

## 📖 目录概览

- [open-snapora Tauri v2 + Rust Demo](#open-snapora-tauri-v2--rust-demo)
  - [📖 目录概览](#-目录概览)
  - [🚀 功能特性](#-功能特性)
  - [💻 环境准备 (Windows)](#-环境准备-windows)
  - [🛠️ 开发调试](#️-开发调试)
  - [🔨 编译生成独立 `.exe` 可执行程序](#-编译生成独立-exe-可执行程序)
    - [1. 执行编译命令](#1-执行编译命令)
    - [2. 编译产物位置](#2-编译产物位置)
    - [3. 静态资源内嵌机制](#3-静态资源内嵌机制)
    - [4. 打包为安装包 (NSIS / MSI)](#4-打包为安装包-nsis--msi)
  - [📂 目录结构](#-目录结构)
  - [📄 License](#-license)

---

## 🚀 功能特性

- **Rust 原生截屏引擎**：集成 `crates/tauri-plugin-snapora`，底层采用 `xcap` 原生直接并发截取各物理屏幕，速度极快、内存开销极小。
- **透明覆盖层窗口**：Tauri 自动管理无边框透明全屏遮罩窗口，在截屏触发瞬间显示，无闪烁。
- **贴图浮窗 (Pin to Desktop)**：截图标注后支持一键钉在桌面上自由拖拽、缩放、置顶。
- **纯 Web 标注工具栏**：支持矩形、椭圆、箭头、画笔、马赛克、文字、撤销/重做及放大镜取色器。

---

## 💻 环境准备 (Windows)

编译 Windows 可执行程序需满足以下基础环境：

1. **Node.js** (>= 18) 与 **pnpm** (>= 8)
2. **Rust 工具链** (建议 Rust 1.75+ 或更高版本，包含 `rustc` 与 `cargo`)
3. **Microsoft C++ Build Tools** (Visual Studio 2022 C++ 桌面开发环境)
4. **WebView2 Runtime** (Windows 10/11 系统通常已自带预装)

---

## 🛠️ 开发调试

在 Monorepo 根目录下或当前目录下运行：

```bash
# 从根目录启动
pnpm demo:tauri

# 或在当前目录运行
pnpm start
# 等同于 pnpm tauri dev
```

首次运行将自动同步最新的 overlay 静态资源、启动 Vite 前端服务，并编译 Rust 原生插件代码。

---

## 🔨 编译生成独立 `.exe` 可执行程序

本项目已预先配置好 Windows 独立绿色版 `.exe` 的打包支持。

### 1. 执行编译命令

在当前目录下执行：

```bash
pnpm build:exe
# 或
pnpm tauri build
```

也可以在 Monorepo 根目录执行：

```bash
pnpm demos:tauri:exe
```

### 2. 编译产物位置

编译成功后，单文件可执行程序将生成在：

```text
demos/tauri/src-tauri/target/release/demo-tauri.exe
```

* **文件体积**：约为 **10 MB** 左右（高度优化并剥离了调试符号）。
* **绿色免安装**：可直接双击运行，也可拷贝到任意相同架构的 Windows 机器上直接使用。

### 3. 静态资源内嵌机制

在构建阶段：
1. `scripts/sync-overlay.mjs` 自动将 `@open-snapora/overlay` 打包产物同步到 `public/overlay`；
2. Vite 将前端主界面与 overlay 资源统一编译打包至 `dist/`；
3. Tauri 编译器将 `dist/` 下的所有 HTML/CSS/JS/图标等静态资源直接内嵌到 Rust 二进制 `.exe` 内部。
因此，运行时无需在 exe 同级目录下放置任何外部静态文件夹即可完整工作。

### 4. 打包为安装包 (NSIS / MSI)

如果需要生成安装程序（如安装向导引导）：
只需编辑 `src-tauri/tauri.conf.json`，将：
```json
"bundle": {
  "active": false
}
```
改为：
```json
"bundle": {
  "active": true,
  "targets": ["nsis", "msi"]
}
```
再运行 `pnpm build:exe`，Tauri 即会自动生成 NSIS / MSI 安装程序。

---

## 📂 目录结构

```text
demos/tauri/
├── package.json
├── index.html               # 演示主页面
├── scripts/
│   └── sync-overlay.mjs     # 遮罩与贴图资源同步脚本
├── src/                     # 前端演示逻辑
├── src-tauri/
│   ├── Cargo.toml           # Rust 依赖 (包含 tauri-plugin-snapora)
│   ├── tauri.conf.json      # Tauri 配置
│   ├── capabilities/        # 权限清单 (包含 snapora 插件权限)
│   └── src/                 # Rust 入口 main.rs
└── vite.config.ts
```

---

## 📄 License

MIT © [open-snapora authors](../../LICENSE)
