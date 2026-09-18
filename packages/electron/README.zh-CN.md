# @open-snapora/electron

> 面向 **Electron** 桌面应用的高性能多屏幕截图、Canvas 标注与桌面置顶贴图管理器。

[English](README.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [Español](README.es.md) | [한국어](README.ko.md)

[![npm version](https://img.shields.io/npm/v/@open-snapora/electron.svg)](https://www.npmjs.com/package/@open-snapora/electron)
[![npm downloads](https://img.shields.io/npm/dm/@open-snapora/electron.svg)](https://www.npmjs.com/package/@open-snapora/electron)
[![bundle size](https://img.shields.io/bundlephobia/minzip/@open-snapora/electron.svg)](https://bundlephobia.com/package/@open-snapora/electron)
[![TypeScript](https://img.shields.io/badge/TypeScript-%E2%9C%93-3178C6.svg)](https://www.typescriptlang.org/)
[![license](https://img.shields.io/npm/l/@open-snapora/electron.svg)](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
[![Electron >= 30](https://img.shields.io/badge/Electron-%3E%3D30.0.0-47848F.svg)](https://www.electronjs.org/)
[![CI](https://github.com/open-toolkits/open-snapora/actions/workflows/ci.yml/badge.svg)](https://github.com/open-toolkits/open-snapora/actions/workflows/ci.yml)

---

## 概述

`@open-snapora/electron` 为 Electron 桌面端应用提供生产级的多显示器屏幕采集、全套交互式标注以及桌面置顶贴图能力。它抹平了跨屏幕全局坐标系换算、HiDPI 高分屏缩放与色彩失真问题，内置零闪烁的透明全屏遮罩缓存复用机制，并提供了兼具安全性与灵活性的 IPC 跨进程通信桥梁。

---

## 核心特性

- 🖥️ **原生多显示器画面捕获**：基于 Electron 底层 `desktopCapturer` 与屏幕服务，精准统一多屏幕逻辑/物理坐标系，无缝适配各类高分屏 Retina 缩放。
- ⚡ **零白屏与窗口缓存复用**：内置智能透明全屏遮罩窗口缓存池，支持静默预加载与实例复用，告别频繁创建 BrowserWindow 的性能损耗与白屏闪烁。
- 🪟 **窗口智能吸附 (Window Snapping)**：鼠标移动时自动探测并吸附高亮当前应用内所有可见窗口边缘；开放 `getWindowSnapRegions` 接口，支持注入操作系统底层原生窗口坐标。
- 📌 **独立置顶贴图浮窗 (Pin to Desktop)**：一键将截图固定到桌面最顶层（Always-on-top）。生成的独立无边框贴图窗口支持鼠标拖拽平移、边缘手柄自由缩放、一键再次复制到剪贴板、另存为本地文件，并自动跟随宿主页面生命周期同步销毁。
- 🎨 **专业级标注画布工具箱**：
  - **几何工具**：矩形、椭圆、箭头、直线。
  - **画笔涂鸦与敏感脱敏**：自由笔触画笔，以及可自由调节马赛克块大小与模糊强度的脱敏滤镜。
  - **富文本与水印**：支持字号、色板、文字描边与背景填充的文本输入，以及半透明防伪水印。
  - **完整历史栈**：支持无限制步骤的撤销与重做（`Undo` / `Redo`）。
  - **精细样式调节**：预设色盘 + 自由 HEX 颜色拾取、线宽滑块、透明度调节。
- 🛡️ **现代企业级安全架构**：
  - 全面基于 **Context Isolation**（上下文隔离），绝不依赖任何不安全的 `remote` 模块。
  - 内置 `validateSender` 校验来源，自动拦截非法跨域 iframe 与未受信任的伪造 IPC 截图请求。
- 🚥 **并发冲突与排队策略 (`busyPolicy`)**：支持 `'queue'`（自动按先进先出 FIFO 队列缓冲并发截图请求）或 `'reject'`（拒绝重复并发点击），可设置队列最大容量 `maxQueuedCaptures`。
- 🔌 **可插拔输出管线 (`outputAdapter`)**：可完全接管输出阶段，将截图二进制流直接上传至阿里云 OSS、腾讯云 COS、AWS S3 或内部业务接口。
- 📊 **阶段级微秒耗时诊断 (`onDiagnostic`)**：提供精确到阶段（`capture` -> `overlay` -> `edit` -> `output`）的微秒级耗时追踪与异常诊断遥测。
- 🌐 **多语言开箱即用 (i18n)**：内置简体中文（`zh-CN`）、英语（`en-US`）、日语（`ja-JP`）、韩语（`ko-KR`）、西班牙语（`es-ES`），支持字段级自定义词条覆盖。
- 🎨 **深浅色主题深度定制**：支持 Dark/Light 双主题，工具栏、遮罩层、选区高亮色与手柄均可自定义配置。

---

## 安装说明

```bash
npm install @open-snapora/electron
# 或者
pnpm add @open-snapora/electron
# 或者
yarn add @open-snapora/electron
```

> **环境依赖**：需要 `electron >= 30.0.0`。

---

## 快速上手（一键集成方案）

### 1. 主进程配置 (`main.js` / `main.ts`)

使用 `setupElectronSnapora` 即可一键完成管理器创建、IPC 通道注册与标准 Preload 路径解析：

```javascript
const { app, BrowserWindow, globalShortcut } = require('electron');
const path = require('path');
const { setupElectronSnapora, resolveHostPreloadPath } = require('@open-snapora/electron');

// 1. 在主进程中初始化 Snapora 控制器
const snapora = setupElectronSnapora({
  busyPolicy: 'queue', // 截图进行中时，后续请求自动排队
});

let mainWindow = null;

function createWindow() {
  mainWindow = new BrowserWindow({
    width: 1024,
    height: 768,
    webPreferences: {
      preload: resolveHostPreloadPath(), // 安全注入 window.electronSnapora
      contextIsolation: true,
      nodeIntegration: false,
    },
  });

  mainWindow.loadFile('index.html');
}

app.whenReady().then(() => {
  createWindow();

  // 2. 注册全局截图快捷键（例如 Windows: Ctrl+Alt+A / macOS: Command+Alt+A）
  const shortcut = process.platform === 'darwin' ? 'Command+Alt+A' : 'Ctrl+Alt+A';
  globalShortcut.register(shortcut, () => {
    // 也可以直接在主进程中发起截图
    snapora.manager.capture({
      locale: 'zh-CN',
      showCopyFeedback: true,
    });
  });
});

app.on('will-quit', () => {
  globalShortcut.unregisterAll();
  snapora.unregister(); // 清理 IPC 监听器与后台缓存窗口
});
```

### 2. 渲染进程调用 (`renderer.js` / 前端框架页面)

页面中通过全局对象 `window.electronSnapora` 即可唤起全功能截屏：

```typescript
async function handleTakeScreenshot() {
  try {
    const result = await window.electronSnapora.capture({
      locale: 'zh-CN',
      showCopyFeedback: true,
      defaultTool: 'select',
      tools: ['rectangle', 'ellipse', 'arrow', 'brush', 'text', 'mosaic', 'watermark'],
      theme: {
        mode: 'dark',
        accentColor: '#3b82f6',
      },
    });

    if (result.status === 'completed') {
      console.log('截图已成功完成！');
      console.log('触发动作:', result.output.action); // 'copy' | 'save' | 'pin'
      console.log('选区边界:', result.bounds);         // { x, y, width, height }
      console.log('屏幕 ID:', result.displayId);
      console.log('图片大小:', result.data.byteLength, 'bytes');

      if (result.output.action === 'save') {
        console.log('文件保存路径:', result.output.filePath);
      }
    } else if (result.status === 'cancelled') {
      console.log('用户取消了截图流程（按 Esc 或取消按钮）');
    } else {
      console.error('截图失败:', result.code, result.message);
    }
  } catch (error) {
    console.error('调用过程捕获到异常:', error);
  }
}
```

---

## 自定义 Preload 集成（进阶方案）

若您的项目已有自定义 Preload 脚本，可通过 `exposeScreenshotApi` 手动接入：

在您的 `preload.ts` 中：

```typescript
import { contextBridge, ipcRenderer } from 'electron';
import { exposeScreenshotApi } from '@open-snapora/electron/preload';

exposeScreenshotApi({
  contextBridge,
  ipcRenderer,
  globalName: 'myScreenshotApi', // 可选，默认为 'electronSnapora'
});
```

在您的前端页面中：

```typescript
const result = await window.myScreenshotApi.capture({
  locale: 'zh-CN',
});
```

---

## API 参考手册

### 主进程导出对象与函数

#### `setupElectronSnapora(options?: SetupElectronSnaporaOptions): SetupElectronSnaporaResult`

一站式集成辅助工具。实例化 `ScreenshotManager`、绑定 IPC 并输出官方 Preload 路径。

```typescript
export interface SetupElectronSnaporaOptions {
  busyPolicy?: 'reject' | 'queue';
  managerOptions?: ScreenshotManagerOptions;
  resourceLimits?: ScreenshotResourceLimitOptions;
  channel?: string;        // 默认: 'plugin:snapora|capture'
  cancelChannel?: string;  // 默认: 'plugin:snapora|cancel'
  validateSender?: (event: IpcMainInvokeEvent) => boolean;
}

export interface SetupElectronSnaporaResult {
  manager: ScreenshotManager;
  preloadPath: string;
  unregister: () => void;
}
```

#### `class ScreenshotManager`

截图底层核心协调器，管理屏幕捕获会话、透明遮罩生命周期、窗口吸附与输出分发。

- `capture(options?: ScreenshotOptions, context?: ScreenshotJobContext): Promise<ScreenshotResult>`
- `cancel(senderWebContentsId?: number): boolean`
- `dispose(): void`

#### `registerScreenshotIpc(options: RegisterScreenshotIpcOptions): () => void`

将 IPC 监听绑定到现有的 `ScreenshotManager` 实例，并返回支持在应用退出或热重载时调用的清理函数。

#### `resolveHostPreloadPath(): string`

返回 `@open-snapora/electron` 内置标准 Preload 文件的绝对路径。

---

### Preload 导出函数 (`@open-snapora/electron/preload`)

#### `exposeScreenshotApi(options: ExposeScreenshotApiOptions): ScreenshotRendererApi`

通过 Electron 的 `contextBridge` 安全地将 `capture()` 和 `cancel()` 挂载到渲染进程的 `window` 对象。

---

### 核心配置与类型定义

#### `ScreenshotOptions` 截图调用选项

| 属性 | 类型 | 默认值 | 描述 |
| :--- | :--- | :--- | :--- |
| `display` | `'cursor' \| 'primary' \| string` | `'cursor'` | 截取目标屏幕：鼠标所在屏、主屏或指定显示器 ID |
| `tools` | `ScreenshotTool[]` | 全部工具 | 工具栏上可见的标注工具列表 |
| `defaultTool` | `'select' \| ScreenshotTool` | `'select'` | 唤起时默认选中的标注工具 |
| `showCopyFeedback` | `boolean` | `false` | 复制到剪贴板成功后是否展示屏幕悬浮 Toast |
| `locale` | `ScreenshotLocale` | `'zh-CN'` | 界面语言：`'en-US'` \| `'zh-CN'` \| `'ja-JP'` \| `'ko-KR'` \| `'es-ES'` |
| `messages` | `ScreenshotMessageOverrides` | `undefined` | 覆盖界面默认词条字典 |
| `theme` | `ScreenshotTheme` | `undefined` | 界面色彩与视觉样式配置 |

#### `ScreenshotResult` 截图结果定义

```typescript
export type ScreenshotResult =
  | (ScreenshotImageResult & { output: ScreenshotOutputMetadata })
  | { status: 'cancelled' }
  | { status: 'failed'; code: ScreenshotErrorCode; message: string };

export interface ScreenshotImageResult {
  status: 'completed';
  data: Uint8Array; // PNG 二进制数据
  mimeType: 'image/png';
  bounds: { x: number; y: number; width: number; height: number };
  displayId: string;
}

export type ScreenshotOutputMetadata =
  | { action: 'copy' }
  | { action: 'save'; filePath: string }
  | { action: 'pin' };
```

---

## 进阶实战案例

### 1. 接管保存逻辑并上传至云存储 (`outputAdapter`)

如果需要将截图直接上传至云端图床而不是写入本地硬盘，可注入自定义 `outputAdapter`：

```typescript
import { ScreenshotManager, registerScreenshotIpc } from '@open-snapora/electron';

const manager = new ScreenshotManager({
  outputAdapter: {
    execute: async (payload, context) => {
      if (payload.action === 'save') {
        // 自定义上传逻辑
        const cloudUrl = await uploadToOSS(payload.result.data);
        return {
          status: 'completed',
          output: { action: 'save', filePath: cloudUrl },
        };
      }
      return null; // 返回 null 则走默认本地存储/剪贴板管线
    },
  },
});
```

### 2. 注入操作系统原生窗口坐标实现全局窗口吸附

默认情况下 Snapora 会吸附当前 Electron 应用内的所有可见窗口。若您集成了原生窗口枚举扩展，可注入 `getWindowSnapRegions`：

```typescript
const snapora = setupElectronSnapora({
  managerOptions: {
    getWindowSnapRegions: () => {
      // 借助 Node 原生扩展获取全局窗口列表
      return [
        { x: 0, y: 0, width: 1920, height: 1080 },
        { x: 100, y: 100, width: 800, height: 600 },
      ];
    },
  },
});
```

### 3. 性能诊断与阶段耗时统计

精确追踪截图全链路耗时，协助排查客户端卡顿：

```typescript
const snapora = setupElectronSnapora({
  managerOptions: {
    onDiagnostic: (event) => {
      console.log(`[Snapora阶段:${event.stage}:${event.phase}] 耗时: ${event.durationMs ?? 0}ms`, event);
    },
  },
});
```

---

## 完整示例

本仓库在 [`demos/electron`](https://github.com/open-toolkits/open-snapora/tree/main/demos/electron) 目录中提供了完整的 Electron 参考工程，欢迎查阅与运行。

---

## 开源协议

MIT © [open-snapora authors](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
