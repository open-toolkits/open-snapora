# Open Snapora

> 🚀 **现代、通用、跨宿主桌面端屏幕截图与图像标注开发套件**  
> (Cross-Framework Desktop Screenshot & Annotation Suite)  
> 统一支持 **Electron**、**Tauri (v2)** 以及各类 **Webview** 桌面应用容器。

---

## 🎯 核心特性

- 🌐 **宿主完全解耦**：核心几何计算、标注文档模型、撤销重做栈与底层桌面容器（Electron / Tauri / Node-Webview）完全隔离，纯 TypeScript 编写，零平台锁定。
- ⚡ **高性能原生截屏**：
  - **Tauri / Rust 模式**：基于 `xcap` 原生直接并发截取全部物理显示器并进行 PNG 编码，内存占用小、截屏速度极快。
  - **Electron 模式**：支持 `desktopCapturer` 实时流提取与全屏离屏捕获双模式，平滑过渡。
- 🎨 **纯 Web 标注视图独立预览**：标注界面与工具栏完全纯前端实现（Canvas / DOM），无需启动桌面宿主即可在普通浏览器中脱机预览和热重载调试。
- 🖥️ **跨平台多显示器无缝融合**：支持多屏幕坐标映射、高分屏 (HiDPI / Retina) 自适应、多屏跨越选区和贴图浮窗（Pin to Desktop）。
- 🔒 **严格的安全通信契约**：内置完备的 `BridgeMessage` 协议与验证器，隔离 Webview 内部与宿主特权能力。
- 📦 **Monorepo 开箱即用**：提供现成可运行的 Electron 示例和 Tauri + Rust 示例。

---

## 📐 架构设计

```text
               宿主业务应用 (Electron / Tauri / Webview)
                                 │
                   统一 API: capture(options) / cancel()
                                 │
                 ┌───────────────┴───────────────┐
                 ▼                               ▼
      @open-snapora/electron            @open-snapora/tauri
     (Electron 宿主适配插件)             (Tauri v2 原生插件与绑定)
                 │                               │
                 └───────────────┬───────────────┘
                                 ▼ 统一宿主桥接契约 (OverlayBridge)
                 ┌───────────────────────────────┐
                 │     @open-snapora/overlay     │
                 │ (纯 Web/Canvas 选区与标注画布) │
                 └───────────────┬───────────────┘
                                 ▼
                 ┌───────────────────────────────┐
                 │     @open-snapora/shared      │
                 │ (几何算法/标注文档模型/历史栈) │
                 └───────────────────────────────┘
```

---

## 📦 项目结构

```text
open-snapora/
├── packages/
│   ├── shared/                # 共享几何、模型、多语言与协议契约 (@open-snapora/shared)
│   ├── overlay/               # 纯 Web 标注编辑器与独立调试页面 (@open-snapora/overlay)
│   ├── electron/              # Electron 宿主插件与多屏控制器 (@open-snapora/electron)
│   └── tauri/                 # Tauri v2 前端绑定 SDK (@open-snapora/tauri)
├── crates/
│   └── tauri-plugin-snapora/  # Tauri v2 原生 Rust 插件 (基于 xcap 截屏)
├── demos/
│   ├── electron/              # Electron 完整集成示例项目
│   └── tauri/                 # Tauri v2 + Rust 完整集成示例项目
├── package.json               # 工作区统一配置
└── pnpm-workspace.yaml
```

---

## 🚀 快速上手与示例运行

### 1. 安装依赖

```bash
pnpm install
```

### 2. 启动示例程序

#### 运行 Electron 示例：
```bash
pnpm demo:electron
```
> 启动后将出现演示窗口，点击 **“开始截屏”** 或按下快捷键 `CommandOrControl+Shift+A` 即可体验全屏截图标注。

#### 运行 Tauri v2 + Rust 示例：
```bash
pnpm demo:tauri
```
> 启动 Tauri 桌面应用（首次启动会自动编译 Rust 原生依赖），点击 **“立即截屏”** 即可体验基于 Rust 原生 `xcap` 驱动的截屏与标注。

#### 纯浏览器独立预览（免编译宿主）：
```bash
pnpm dev:overlay
```
> 启动 Vite 服务并在浏览器打开 `http://localhost:5173/dev.html`，可在内置测试底图上快速调试选区、矩形、箭头、马赛克、画笔、文字与撤销重做功能。

---

## 💻 宿主集成指南

### 1. 在 Electron 项目中集成

#### 安装依赖：
```bash
pnpm add @open-snapora/electron
```

#### 主进程集成代码：
```javascript
const { app, BrowserWindow, globalShortcut } = require('electron');
const { setupSnaporaMain } = require('@open-snapora/electron');

let snaporaController = null;

app.whenReady().then(() => {
  // 初始化 Snapora 控制器并监听截屏完成/取消事件
  snaporaController = setupSnaporaMain({
    onComplete: (data) => {
      console.log('截屏完成，文件保存在:', data.outputPath);
    },
    onCancel: (reason) => {
      console.log('截屏取消:', reason);
    },
  });

  // 注册全局快捷键触发截屏
  globalShortcut.register('CommandOrControl+Shift+A', () => {
    snaporaController.startCapture();
  });
});
```

---

### 2. 在 Tauri v2 项目中集成

#### 添加依赖：
在前端项目中安装：
```bash
pnpm add @open-snapora/tauri
```

在 `src-tauri/Cargo.toml` 中引入插件：
```toml
[dependencies]
tauri = { version = "2.0", features = ["wry"] }
tauri-plugin-snapora = { path = "../crates/tauri-plugin-snapora" } # 或使用 crates.io 依赖
```

#### Rust 注册插件 (`src-tauri/src/main.rs`)：
```rust
fn main() {
    tauri::Builder::default()
        // 注册 Snapora Tauri 原生插件
        .plugin(tauri_plugin_snapora::init())
        .run(tauri::generate_context!())
        .expect("启动应用失败");
}
```

#### 前端页面调用：
```typescript
import { capture, cancel } from '@open-snapora/tauri';

// 点击按钮触发截屏
async function handleCapture() {
  try {
    const result = await capture({
      hideOwnerWindow: true, // 截屏前自动隐藏宿主窗口
    });
    console.log('截屏成功，生成数据:', result);
  } catch (err) {
    console.log('截屏取消或出错:', err);
  }
}
```

---

### 3. 在自定义宿主 (Webview / Node-Webview) 中集成

`@open-snapora/overlay` 提供了标准的显式挂载入口：
```typescript
import { mountOverlay } from '@open-snapora/overlay';
import type { OverlayBridge, HostFrame } from '@open-snapora/shared';

// 1. 实现您当前宿主的桥接接口
const customBridge: OverlayBridge = {
  getInitState: async () => ({ ... }),
  ready: () => { ... },
  confirm: (payload) => { ... },
  cancel: (reason) => { ... },
  pin: (payload) => { ... },
  // ... 其余方法
};

// 2. 挂载到指定 DOM 容器
const handle = mountOverlay({
  container: document.getElementById('snapora-root')!,
  bridge: customBridge,
  loadFrame: async (screenId) => {
    // 提供该屏幕的 ImageData 或 ImageBitmap
    return myNativeGetFrame(screenId);
  }
});
```

---

## 🛠️ 常用开发脚本

| 命令 | 描述 |
| :--- | :--- |
| `pnpm run check:all` | **一键全量校验**：执行 TS 类型检查、包构建、Rust 插件编译与示例校验 |
| `pnpm run typecheck` | 执行工作区内所有包的 TypeScript 类型检查 |
| `pnpm run build` | 构建 `packages/*` 下的所有子包 (Shared / Overlay / Electron / Tauri) |
| `pnpm demos:electron` | 启动 Electron 集成示例应用 (`demos/electron`) |
| `pnpm demos:tauri` | 启动 Tauri v2 + Rust 集成示例应用 (`demos/tauri`) |
| `pnpm run demos:tauri:check` | 针对 Tauri 演示项目的 Rust 后端进行 `cargo check` 编译检查 |
| `pnpm dev:overlay` | 启动纯 Web 标注画布脱机开发服务器 (`dev.html`) |
| `pnpm run clean` | 清理工作区内所有 `dist/` 产物目录 |

---

## 📄 开源许可

本项目遵循 [MIT License](./LICENSE) 开源协议。欢迎提交 PR 与 Issue！

