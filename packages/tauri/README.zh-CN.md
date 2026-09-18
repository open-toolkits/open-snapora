# @open-snapora/tauri

> 面向 **Tauri (v2)** 的高性能跨平台桌面截屏、标注与桌面置顶贴图套件。

[English](README.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [Español](README.es.md) | [한국어](README.ko.md)

[![npm version](https://img.shields.io/npm/v/@open-snapora/tauri.svg)](https://www.npmjs.com/package/@open-snapora/tauri)
[![npm downloads](https://img.shields.io/npm/dm/@open-snapora/tauri.svg)](https://www.npmjs.com/package/@open-snapora/tauri)
[![bundle size](https://img.shields.io/bundlephobia/minzip/@open-snapora/tauri.svg)](https://bundlephobia.com/package/@open-snapora/tauri)
[![TypeScript](https://img.shields.io/badge/TypeScript-%E2%9C%93-3178C6.svg)](https://www.typescriptlang.org/)
[![license](https://img.shields.io/npm/l/@open-snapora/tauri.svg)](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
[![Tauri v2](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://v2.tauri.app)
[![CI](https://github.com/open-toolkits/open-snapora/actions/workflows/ci.yml/badge.svg)](https://github.com/open-toolkits/open-snapora/actions/workflows/ci.yml)

---

## 概述

`@open-snapora/tauri` 是 open-snapora 专为 Tauri v2 生态打造的官方客户端适配器。基于原生 Rust 插件 [`tauri-plugin-snapora`](https://github.com/open-toolkits/open-snapora/tree/main/crates/tauri-plugin-snapora) 与高效底层引擎 `xcap`，在毫秒级内完成跨屏幕物理与逻辑坐标的画面采集，提供功能完备的交互式 Canvas 标注遮罩，以及支持拖拽、缩放的无边框桌面置顶贴图（Pinned Window），资源占用极低且启动极其轻快。

---

## 核心功能特性

- ⚡ **原生 Rust 捕获引擎**：底层基于 `xcap` 硬件加速采集，自动换算多显示器空间坐标与 HiDPI 缩放比例，不丢帧、不失真、内存开销极小。
- 🚀 **后台静默预热 (`prewarm`)**：支持在主应用启动后静默预先加载透明遮罩窗口与 WebView2 运行时，彻底解决首次截图白屏与冷启动卡顿，实现即点即开的秒级响应。
- 🪟 **窗口智能吸附 (Window Snapping)**：鼠标悬停时自动探测系统活动窗口边界并精准吸附高亮，单击即可快速框选目标窗口。
- 🎨 **全能标注工具箱**：
  - **几何图形**：矩形、椭圆、箭头、直线等。
  - **涂鸦与脱敏**：自由笔触画笔，以及可调节强度的马赛克（Mosaic）与模糊滤镜，轻松隐藏敏感个人信息。
  - **文字与标记**：可自由编辑的富文本输入（支持字号、色彩、外描边、背景填充），以及防盗图半透明水印。
  - **完整历史记录**：全链路多步撤销与重做（`Undo` / `Redo`）。
  - **精细化调节**：可视化调色板（预置经典色盘 + 自定义 HEX）、笔触粗细滑动条与透明度控制。
- 📌 **桌面置顶贴图 (Pin to Desktop)**：一键将截图固定在桌面最顶层（Always-on-top）。浮窗支持自由拖拽平移、边缘把手调整尺寸、再次一键复制到剪贴板、另存为本地文件以及随时关闭。
- 📋 **多元化输出管线**：
  - 复制到系统剪贴板（写入标准 PNG 位图，支持复制成功浮动 Toast 反馈 `showCopyFeedback`）。
  - 另存为本地文件（调起原生保存对话框，返回最终绝对路径 `filePath`）。
  - 置顶贴图至桌面独立浮窗。
- 🌐 **开箱即用的多语言 (i18n)**：原生内置 5 种语言包：`zh-CN`（简体中文）、`en-US`（英语）、`ja-JP`（日语）、`ko-KR`（韩语）、`es-ES`（西班牙语），并开放字段级文案全量重写。
- 🎨 **深度视觉主题定制**：支持明暗双色模式（Dark / Light），可自由配置强调色、遮罩半透明度、工具栏底色、边框色、悬停高亮与选区手柄色彩。
- 🛡️ **Tauri v2 权限安全规范**：严密契合 Tauri v2 Capabilities 细粒度权限模型，默认配置即开即用，保障宿主进程安全。

---

## 安装与环境依赖

### 1. 前置依赖要求

- **前端运行时**：`@tauri-apps/api >= 2.0.0`
- **Rust 后端**：Tauri v2 (`tauri >= 2.0`)，Rust `>= 1.77.2`

### 2. 安装 NPM 依赖包

```bash
npm install @open-snapora/tauri
# 或者
pnpm add @open-snapora/tauri
# 或者
yarn add @open-snapora/tauri
```

### 3. 配置 Rust 插件依赖

在您的 Tauri 项目 `src-tauri/Cargo.toml` 中添加：

```toml
[dependencies]
tauri = { version = "2.0", features = ["wry"] }
tauri-plugin-snapora = { git = "https://github.com/open-toolkits/open-snapora.git" }
```

---

## 快速上手

### 第 1 步：声明权限 Capabilities

在 Tauri 的权限配置文件（如 `src-tauri/capabilities/default.json`）中加入 `snapora:default`：

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "identifier": "default",
  "windows": ["*"],
  "permissions": [
    "core:default",
    "snapora:default"
  ]
}
```

### 第 2 步：在 Rust 中注册插件

在 `src-tauri/src/lib.rs`（或 `main.rs`）中注册：

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_snapora::init())
        .run(tauri::generate_context!())
        .expect("运行 Tauri 应用时发生错误");
}
```

### 第 3 步：前端发起截图

```typescript
import { capture, prewarm, type ScreenshotResult } from '@open-snapora/tauri';

// 建议：在应用主窗口就绪后在后台静默预热，后续截图将直接毫秒级秒开
void prewarm().catch(console.error);

async function handleTakeScreenshot() {
  try {
    const result: ScreenshotResult = await capture({
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
      console.log('截图成功完成！');
      console.log('执行动作:', result.output.action); // 'copy' | 'save' | 'pin'
      console.log('截图选区:', result.bounds);         // { x, y, width, height }
      console.log('所在屏幕:', result.displayId);
      console.log('PNG 字节大小:', result.data.byteLength);

      if (result.output.action === 'save') {
        console.log('文件保存路径:', result.output.filePath);
      }
    } else if (result.status === 'cancelled') {
      console.log('用户取消了截图流程（按下了 Esc 或点击取消按钮）');
    } else {
      console.error('截图失败:', result.code, result.message);
    }
  } catch (error) {
    console.error('调用过程捕获到未知异常:', error);
  }
}
```

---

## API 参考手册

### 导出函数

#### `capture(options?: ScreenshotOptions): Promise<ScreenshotResult>`

启动全屏截图会话，激活交互式标注遮罩层，并异步等待用户的完成确认、取消或异常退出。

#### `cancel(): Promise<boolean>`

以编程方式取消当前正在进行的截图会话。若有活跃会话被取消则返回 `true`。

#### `prewarm(): Promise<void>`

在后台静默初始化隐藏的透明遮罩窗口和 WebView2 环境。强烈建议在主窗口加载完成后尽早调用。

#### `createTauriOverlayBridge(): ScreenshotOverlayApi`

底层工厂方法，供定制或二次开发标注遮罩 Canvas 页面的开发者使用，负责维护与 Rust 原生端的双向通信。

#### `createTauriPinnedBridge(): PinnedImageApi`

底层工厂方法，供定制置顶贴图浮窗 UI 的开发者使用，负责拖拽、缩放、保存等事件的桥接。

---

### 类型定义

#### `ScreenshotOptions` 截图选项配置

| 属性 | 类型 | 默认值 | 描述 |
| :--- | :--- | :--- | :--- |
| `display` | `'cursor' \| 'primary' \| string` | `'cursor'` | 截取的目标显示器：当前鼠标所在屏、主屏或特定屏幕 ID |
| `tools` | `ScreenshotTool[]` | 全部工具 | 工具栏上启用的标注工具清单 |
| `defaultTool` | `'select' \| ScreenshotTool` | `'select'` | 启动时默认激活的工具 |
| `showCopyFeedback` | `boolean` | `false` | 复制到剪贴板成功后是否在屏幕居中显示浮动反馈 Toast |
| `locale` | `ScreenshotLocale` | `'zh-CN'` | 界面多语言：`'en-US'` \| `'zh-CN'` \| `'ja-JP'` \| `'ko-KR'` \| `'es-ES'` |
| `messages` | `ScreenshotMessageOverrides` | `undefined` | 自定义文案字典覆盖，用于替换界面上的任意提示和按钮文字 |
| `theme` | `ScreenshotTheme` | `undefined` | 界面主题色系与视觉样式配置 |

```typescript
export type ScreenshotTool =
  | 'rectangle'
  | 'ellipse'
  | 'arrow'
  | 'brush'
  | 'text'
  | 'mosaic'
  | 'watermark';
```

#### `ScreenshotResult` 截图结果定义

```typescript
export type ScreenshotResult =
  | (ScreenshotImageResult & { output: ScreenshotOutputMetadata })
  | { status: 'cancelled' }
  | { status: 'failed'; code: ScreenshotErrorCode; message: string };

export interface ScreenshotImageResult {
  status: 'completed';
  data: Uint8Array; // 纯 PNG 二进制数据
  mimeType: 'image/png';
  bounds: ScreenshotBounds; // { x: number; y: number; width: number; height: number }
  displayId: string;
}

export type ScreenshotOutputMetadata =
  | { action: 'copy' }
  | { action: 'save'; filePath: string }
  | { action: 'pin' };
```

#### `ScreenshotTheme` 主题配置

```typescript
export interface ScreenshotTheme {
  mode?: 'dark' | 'light';
  accentColor?: string;
  accentForegroundColor?: string;
  maskColor?: string;
  toolbarBackground?: string;
  toolbarForeground?: string;
  toolbarBorderColor?: string;
  toolbarHoverBackground?: string;
  tooltipBackground?: string;
  tooltipForeground?: string;
  destructiveColor?: string;
  warningColor?: string;
  warningForegroundColor?: string;
  selectionHandleColor?: string;
  copyFeedbackBackground?: string;
  copyFeedbackForeground?: string;
  copyFeedbackBorderColor?: string;
  copyFeedbackIconColor?: string;
  copyFeedbackIconBackground?: string;
}
```

#### `ScreenshotErrorCode` 错误码枚举

| 错误代码 | 含义 |
| :--- | :--- |
| `CAPTURE_BUSY` | 已有截图任务正在进行中，请勿频繁重复发起 |
| `INVALID_REQUEST` | 传递的选项参数格式无效或非法 |
| `RESOURCE_LIMIT_EXCEEDED` | 截取的画面尺寸或数据大小超出了系统安全阈值 |
| `PERMISSION_DENIED` | 系统未授予屏幕录制权限（常见于 macOS 权限弹窗） |
| `DISPLAY_NOT_FOUND` | 未检测到指定的显示器设备 |
| `CAPTURE_FAILED` | 底层 Rust 截图引擎抓取画面失败 |
| `OVERLAY_LOAD_FAILED` | 透明标注遮罩页面加载超时或渲染失败 |
| `EXPORT_FAILED` | 图像编码或写入剪贴板/文件失败 |
| `UNSUPPORTED_PLATFORM` | 当前操作系统平台不受支持 |

---

## 进阶实战技巧

### 1. 全局快捷键唤起截图

借助官方插件 `@tauri-apps/plugin-global-shortcut`，可轻松实现全局快捷键唤醒：

```typescript
import { register } from '@tauri-apps/plugin-global-shortcut';
import { capture } from '@open-snapora/tauri';

await register('CommandOrControl+Shift+A', async (event) => {
  if (event.state === 'Pressed') {
    await capture({ showCopyFeedback: true });
  }
});
```

### 2. 自定义文案与提示

您可以单独覆盖某个按钮的文案，而无需重新配置整套语言表：

```typescript
await capture({
  locale: 'zh-CN',
  messages: {
    instruction: '按住鼠标左键框选区域，按 Esc 键随时退出',
    copy: '复制到剪贴板',
    save: '另存为图片',
    pin: '置顶到桌面',
  },
});
```

### 3. Tauri 权限拆解清单

默认权限集合 `snapora:default` 内部打包了以下细分指令权限：

| 权限标识 | 功能描述 |
| :--- | :--- |
| `allow-capture` | 允许发起屏幕抓取并激活会话 |
| `allow-cancel-active` | 允许取消当前活跃的截图会话 |
| `allow-prewarm` | 允许在后台预热透明遮罩窗口 |
| `allow-output` | 允许执行复制、保存与贴图等结果输出 |
| `allow-pinned-*` | 允许针对置顶贴图浮窗进行拖拽移动、缩放、复制与关闭 |

---

## 示例应用

完整可运行的 Tauri v2 示例工程位于本仓库的 [`demos/tauri`](https://github.com/open-toolkits/open-snapora/tree/main/demos/tauri) 目录中。

---

## 开源协议

MIT © [open-snapora authors](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
