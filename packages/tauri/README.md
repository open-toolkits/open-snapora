# @open-snapora/tauri

> High-performance cross-platform desktop screenshot, annotation, and desktop-pinning toolkit for **Tauri (v2)**.

[English](README.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [Español](README.es.md) | [한국어](README.ko.md)

[![npm version](https://img.shields.io/npm/v/@open-snapora/tauri.svg)](https://www.npmjs.com/package/@open-snapora/tauri)
[![npm downloads](https://img.shields.io/npm/dm/@open-snapora/tauri.svg)](https://www.npmjs.com/package/@open-snapora/tauri)
[![bundle size](https://img.shields.io/bundlephobia/minzip/@open-snapora/tauri.svg)](https://bundlephobia.com/package/@open-snapora/tauri)
[![TypeScript](https://img.shields.io/badge/TypeScript-%E2%9C%93-3178C6.svg)](https://www.typescriptlang.org/)
[![license](https://img.shields.io/npm/l/@open-snapora/tauri.svg)](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
[![Tauri v2](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://v2.tauri.app)
[![CI](https://github.com/open-toolkits/open-snapora/actions/workflows/ci.yml/badge.svg)](https://github.com/open-toolkits/open-snapora/actions/workflows/ci.yml)

---

## Overview

`@open-snapora/tauri` is the official client adapter for integrating open-snapora with Tauri v2 applications. Backed by the native Rust crate [`tauri-plugin-snapora`](https://github.com/open-toolkits/open-snapora/tree/main/crates/tauri-plugin-snapora), it delivers lightning-fast screen capture, an interactive full-featured canvas annotation overlay, and native floating pinned windows with minimal resource footprint.

---

## Key Features

- ⚡ **Native Rust Capture Engine**: Hardware-accelerated multi-display screen capture powered by Rust `xcap`. Accurately maps physical coordinates, multi-monitor setups, and HiDPI displays.
- 🚀 **Silent Background Prewarming (`prewarm`)**: Pre-warms transparent overlay windows and the WebView2 runtime on startup. Eliminates cold-boot delays and blank-screen flashes on user trigger.
- 🪟 **Smart Window Snapping**: Automatically detects and snaps to active window boundaries for seamless single-window selection.
- 🎨 **Comprehensive Annotation Suite**:
  - **Shapes**: Rectangles, ellipses, arrows, and straight lines.
  - **Drawing & Blurring**: Freehand brush and customizable pixelated mosaic / blur tools for obscuring sensitive data.
  - **Typography & Marking**: Rich text editing (custom fonts, sizes, outline, fill) and customizable watermarks.
  - **Full History**: Complete multi-step undo and redo (`Undo` / `Redo`).
  - **Precision Control**: Visual color picker (preset palette + HEX input), adjustable stroke widths, and opacity sliders.
- 📌 **Pin to Desktop (Floating Window)**: Pin any captured snapshot as a frameless, always-on-top, draggable, and resizable desktop widget. Supports one-click re-copying, local saving, and closing.
- 📋 **Flexible Output Pipelines**:
  - Direct copy to system clipboard with visual toast feedback (`showCopyFeedback`).
  - Native save file dialog returning the absolute `filePath`.
  - Pin snapshot directly to the desktop.
- 🌐 **Built-in Multi-language (i18n)**: Out-of-the-box support for `en-US`, `zh-CN`, `ja-JP`, `ko-KR`, and `es-ES`, plus full string-level override capabilities.
- 🎨 **Deep Visual Customization**: Switch between dark and light themes, or customize accent colors, masks, toolbar styles, and control handles.
- 🛡️ **Tauri v2 Security Compliant**: Strictly follows Tauri v2 permissions and capabilities architecture.

---

## Installation & Requirements

### 1. Prerequisites

- **Frontend**: `@tauri-apps/api >= 2.0.0`
- **Rust Backend**: Tauri v2 (`tauri >= 2.0`)

### 2. Install NPM Package

```bash
npm install @open-snapora/tauri
# or
pnpm add @open-snapora/tauri
# or
yarn add @open-snapora/tauri
```

### 3. Add Rust Plugin Dependency

In your `src-tauri/Cargo.toml`:

```toml
[dependencies]
tauri = { version = "2.0", features = ["wry"] }
tauri-plugin-snapora = { git = "https://github.com/open-toolkits/open-snapora.git" }
```

---

## Quick Start

### Step 1: Configure Capabilities

Add `snapora:default` to your Tauri capabilities file (e.g. `src-tauri/capabilities/default.json`):

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

### Step 2: Register the Plugin in Rust

In `src-tauri/src/lib.rs` (or `main.rs`):

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_snapora::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Step 3: Trigger Capture from Frontend

```typescript
import { capture, prewarm, type ScreenshotResult } from '@open-snapora/tauri';

// Optional: Prewarm overlay window on startup to achieve sub-second response
void prewarm().catch(console.error);

async function handleTakeScreenshot() {
  try {
    const result: ScreenshotResult = await capture({
      locale: 'en-US',
      showCopyFeedback: true,
      defaultTool: 'select',
      tools: ['rectangle', 'ellipse', 'arrow', 'brush', 'text', 'mosaic', 'watermark'],
      theme: {
        mode: 'dark',
        accentColor: '#3b82f6',
      },
    });

    if (result.status === 'completed') {
      console.log('Capture succeeded!');
      console.log('Action taken:', result.output.action); // 'copy' | 'save' | 'pin'
      console.log('Bounding box:', result.bounds);         // { x, y, width, height }
      console.log('Target display:', result.displayId);
      console.log('PNG bytes length:', result.data.byteLength);

      if (result.output.action === 'save') {
        console.log('Saved to file:', result.output.filePath);
      }
    } else if (result.status === 'cancelled') {
      console.log('User cancelled the screenshot (Esc or Cancel button).');
    } else {
      console.error('Capture failed:', result.code, result.message);
    }
  } catch (error) {
    console.error('Unexpected error during capture:', error);
  }
}
```

---

## API Reference

### Methods

#### `capture(options?: ScreenshotOptions): Promise<ScreenshotResult>`

Launches the full-screen screenshot session, displays the annotation canvas overlay, and waits for user confirmation, cancellation, or error.

#### `cancel(): Promise<boolean>`

Cancels the active screenshot session programmatically. Returns `true` if a session was active and cancelled.

#### `prewarm(): Promise<void>`

Silently initializes the hidden transparent overlay window and WebView2 runtime in the background. Highly recommended to invoke shortly after the main window is loaded.

#### `createTauriOverlayBridge(): ScreenshotOverlayApi`

Low-level factory for building custom overlay canvas pages that communicate with the Rust backend.

#### `createTauriPinnedBridge(): PinnedImageApi`

Low-level factory for building custom pinned floating windows.

---

### Type Definitions

#### `ScreenshotOptions`

```typescript
export interface ScreenshotOptions {
  /** Target monitor: 'cursor' (default), 'primary', or specific display ID */
  display?: 'cursor' | 'primary' | string;

  /** Visible annotation tools on the toolbar */
  tools?: ScreenshotTool[];

  /** Tool selected by default on launch */
  defaultTool?: 'select' | ScreenshotTool;

  /** Display a floating toast banner when an image is copied to clipboard (default: false) */
  showCopyFeedback?: boolean;

  /** Language code: 'en-US' | 'zh-CN' | 'ja-JP' | 'ko-KR' | 'es-ES' */
  locale?: ScreenshotLocale;

  /** Custom text overrides for UI labels and tooltips */
  messages?: ScreenshotMessageOverrides;

  /** Custom visual theme configurations */
  theme?: ScreenshotTheme;
}

export type ScreenshotTool =
  | 'rectangle'
  | 'ellipse'
  | 'arrow'
  | 'brush'
  | 'text'
  | 'mosaic'
  | 'watermark';
```

#### `ScreenshotResult`

```typescript
export type ScreenshotResult =
  | (ScreenshotImageResult & { output: ScreenshotOutputMetadata })
  | { status: 'cancelled' }
  | { status: 'failed'; code: ScreenshotErrorCode; message: string };

export interface ScreenshotImageResult {
  status: 'completed';
  data: Uint8Array; // Raw PNG bytes
  mimeType: 'image/png';
  bounds: ScreenshotBounds; // { x, y, width, height }
  displayId: string;
}

export type ScreenshotOutputMetadata =
  | { action: 'copy' }
  | { action: 'save'; filePath: string }
  | { action: 'pin' };
```

#### `ScreenshotTheme`

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

#### `ScreenshotErrorCode`

```typescript
export type ScreenshotErrorCode =
  | 'CAPTURE_BUSY'
  | 'INVALID_REQUEST'
  | 'RESOURCE_LIMIT_EXCEEDED'
  | 'PERMISSION_DENIED'
  | 'DISPLAY_NOT_FOUND'
  | 'CAPTURE_FAILED'
  | 'OVERLAY_LOAD_FAILED'
  | 'EXPORT_FAILED'
  | 'INVALID_RESULT'
  | 'UNSUPPORTED_PLATFORM';
```

---

## Advanced Usage

### 1. Global Keyboard Shortcut Integration

You can integrate global shortcuts via `@tauri-apps/plugin-global-shortcut`:

```typescript
import { register } from '@tauri-apps/plugin-global-shortcut';
import { capture } from '@open-snapora/tauri';

await register('CommandOrControl+Shift+A', async (event) => {
  if (event.state === 'Pressed') {
    await capture({ showCopyFeedback: true });
  }
});
```

### 2. Custom Tool Labels & I18n

Override specific labels without redefining the entire dictionary:

```typescript
await capture({
  locale: 'en-US',
  messages: {
    instruction: 'Click and drag to select an area. Press Esc to exit.',
    copy: 'Copy Image',
    save: 'Export As...',
    pin: 'Pin to Screen',
  },
});
```

### 3. Tauri Permissions Breakdown

The default permission set `snapora:default` bundles the following command permissions:

| Permission | Description |
| :--- | :--- |
| `allow-capture` | Launch screen capture session |
| `allow-cancel-active` | Cancel active capture session |
| `allow-prewarm` | Background window prewarming |
| `allow-output` | Output action processing (copy, save, pin) |
| `allow-pinned-*` | Pinned floating window manipulation (drag, resize, copy, save, close) |

---

## Example Demo

Check out the full working Tauri v2 demo application in the [`demos/tauri`](https://github.com/open-toolkits/open-snapora/tree/main/demos/tauri) directory.

---

## License

MIT © [open-snapora authors](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
