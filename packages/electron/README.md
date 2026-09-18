# @open-snapora/electron

> Comprehensive multi-display screen capture, canvas annotation, and desktop-pinning manager for **Electron**.

[English](README.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [Español](README.es.md) | [한국어](README.ko.md)

[![npm version](https://img.shields.io/npm/v/@open-snapora/electron.svg)](https://www.npmjs.com/package/@open-snapora/electron)
[![npm downloads](https://img.shields.io/npm/dm/@open-snapora/electron.svg)](https://www.npmjs.com/package/@open-snapora/electron)
[![bundle size](https://img.shields.io/bundlephobia/minzip/@open-snapora/electron.svg)](https://bundlephobia.com/package/@open-snapora/electron)
[![TypeScript](https://img.shields.io/badge/TypeScript-%E2%9C%93-3178C6.svg)](https://www.typescriptlang.org/)
[![license](https://img.shields.io/npm/l/@open-snapora/electron.svg)](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
[![Electron >= 30](https://img.shields.io/badge/Electron-%3E%3D30.0.0-47848F.svg)](https://www.electronjs.org/)
[![CI](https://github.com/open-toolkits/open-snapora/actions/workflows/ci.yml/badge.svg)](https://github.com/open-toolkits/open-snapora/actions/workflows/ci.yml)

---

## Overview

`@open-snapora/electron` provides production-grade screen capture and visual annotation infrastructure for Electron desktop applications. It coordinates multi-monitor coordinate spaces, handles HiDPI scaling, manages transparent overlay windows with zero-flicker caching, provides floating desktop pinned widgets, and enforces strict security isolation between Electron main and renderer processes.

---

## Key Features

- 🖥️ **Native Multi-Display Capture**: Seamlessly unifies multi-monitor physical bounds and handles cross-display coordinate transformations and HiDPI retina scaling.
- ⚡ **Zero-Flicker Window Caching**: Caches and reuses hidden transparent overlay windows to deliver instant capture response without white flashes or window creation overhead.
- 🪟 **Smart Window Snapping (`getWindowSnapRegions`)**: Automatically highlights and snaps to visible app windows on hover. Supports custom injection of native OS window rects.
- 📌 **Native Floating Pinned Windows (Pin to Desktop)**: Pin snapshots into frameless, draggable, resizable, always-on-top floating windows. Includes built-in copy, save, and close actions with automatic lifecycle garbage collection.
- 🎨 **Full-Featured Canvas Annotation Suite**:
  - **Geometric Tools**: Rectangles, ellipses, arrows, and lines.
  - **Drawing & Privacy**: Freehand brush and adjustable pixelated mosaic / blur tools.
  - **Typography & Marking**: Rich text labels (font size, color, stroke outline, fill) and customizable watermarks.
  - **Editing History**: Full multi-step Undo and Redo (`Undo` / `Redo`).
  - **Styling**: Preset palette + HEX color picker, stroke width slider, and opacity adjustment.
- 🛡️ **Enterprise-Grade Security Architecture**:
  - Requires **Context Isolation** (`contextIsolation: true`). No `remote` module needed.
  - Built-in sender validation (`validateSender`) blocks unauthorized iframes and malicious IPC origins.
- 🚥 **Concurrency & Queue Policy (`busyPolicy`)**: Choose between `'queue'` (FIFO queuing for rapid repeated captures) or `'reject'` (instant busy rejection), with configurable `maxQueuedCaptures`.
- 🔌 **Pluggable Output Pipeline (`outputAdapter`)**: Take full control of the export step to stream captured buffers directly to S3, OSS, internal APIs, or custom disk paths.
- 📊 **Granular Diagnostic Telemetry (`onDiagnostic`)**: Capture phase-by-phase microsecond timings and diagnostic metrics across `capture`, `overlay`, `edit`, and `output`.
- 🌐 **Built-in Multi-language (i18n)**: Out-of-the-box support for `en-US`, `zh-CN`, `ja-JP`, `ko-KR`, and `es-ES`, plus full custom string overrides.
- 🎨 **Deep Theming**: Dark and light mode presets, customizable accent colors, masks, toolbar palettes, and selection handles.

---

## Installation

```bash
npm install @open-snapora/electron
# or
pnpm add @open-snapora/electron
# or
yarn add @open-snapora/electron
```

> **Peer Dependency**: Requires `electron >= 30.0.0`.

---

## Quick Start (One-Line Setup)

### 1. Main Process (`main.js` / `main.ts`)

Use `setupElectronSnapora` to initialize the manager, register IPC handlers, and obtain the pre-bundled preload script path:

```javascript
const { app, BrowserWindow, globalShortcut } = require('electron');
const path = require('path');
const { setupElectronSnapora, resolveHostPreloadPath } = require('@open-snapora/electron');

// 1. Initialize Snapora controller in the main process
const snapora = setupElectronSnapora({
  busyPolicy: 'queue', // Queue concurrent capture requests
});

let mainWindow = null;

function createWindow() {
  mainWindow = new BrowserWindow({
    width: 1024,
    height: 768,
    webPreferences: {
      preload: resolveHostPreloadPath(), // Injects window.electronSnapora safely
      contextIsolation: true,
      nodeIntegration: false,
    },
  });

  mainWindow.loadFile('index.html');
}

app.whenReady().then(() => {
  createWindow();

  // 2. Register global hotkey (e.g. Ctrl+Alt+A or Command+Alt+A)
  const shortcut = process.platform === 'darwin' ? 'Command+Alt+A' : 'Ctrl+Alt+A';
  globalShortcut.register(shortcut, () => {
    // Directly invoke capture from the main process
    snapora.manager.capture({
      locale: 'en-US',
      showCopyFeedback: true,
    });
  });
});

app.on('will-quit', () => {
  globalShortcut.unregisterAll();
  snapora.unregister(); // Clean up IPC handlers and overlay windows
});
```

### 2. Renderer Process (`renderer.js` / Frontend Framework)

With `resolveHostPreloadPath()`, `window.electronSnapora` is automatically exposed in the DOM:

```typescript
// Call from your Vue, React, or vanilla JS application
async function triggerScreenshot() {
  try {
    const result = await window.electronSnapora.capture({
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
      console.log('Action:', result.output.action); // 'copy' | 'save' | 'pin'
      console.log('Bounds:', result.bounds);         // { x, y, width, height }
      console.log('Display ID:', result.displayId);
      console.log('Image bytes:', result.data.byteLength);

      if (result.output.action === 'save') {
        console.log('Saved file path:', result.output.filePath);
      }
    } else if (result.status === 'cancelled') {
      console.log('Capture cancelled by user.');
    } else {
      console.error('Capture failed:', result.code, result.message);
    }
  } catch (error) {
    console.error('Failed to trigger capture:', error);
  }
}
```

---

## Custom Preload Setup (Advanced)

If your application already uses a custom preload script, you can expose the API manually using `exposeScreenshotApi`:

In your `preload.ts`:

```typescript
import { contextBridge, ipcRenderer } from 'electron';
import { exposeScreenshotApi } from '@open-snapora/electron/preload';

exposeScreenshotApi({
  contextBridge,
  ipcRenderer,
  globalName: 'myScreenshotApi', // Optional, defaults to 'electronSnapora'
});
```

In your frontend:

```typescript
const result = await window.myScreenshotApi.capture({
  locale: 'en-US',
});
```

---

## API Reference

### Main Process Exports

#### `setupElectronSnapora(options?: SetupElectronSnaporaOptions): SetupElectronSnaporaResult`

All-in-one initialization helper. Instantiates `ScreenshotManager`, registers host IPC handlers, and resolves the pre-bundled host preload script.

```typescript
export interface SetupElectronSnaporaOptions {
  busyPolicy?: 'reject' | 'queue';
  managerOptions?: ScreenshotManagerOptions;
  resourceLimits?: ScreenshotResourceLimitOptions;
  channel?: string;        // Default: 'plugin:snapora|capture'
  cancelChannel?: string;  // Default: 'plugin:snapora|cancel'
  validateSender?: (event: IpcMainInvokeEvent) => boolean;
}

export interface SetupElectronSnaporaResult {
  manager: ScreenshotManager;
  preloadPath: string;
  unregister: () => void;
}
```

#### `class ScreenshotManager`

The core orchestrator responsible for screen capture, overlay window lifecycle, window snapping, and output dispatch.

- `capture(options?: ScreenshotOptions, context?: ScreenshotJobContext): Promise<ScreenshotResult>`
- `cancel(senderWebContentsId?: number): boolean`
- `dispose(): void`

#### `registerScreenshotIpc(options: RegisterScreenshotIpcOptions): () => void`

Binds IPC handlers to an existing `ScreenshotManager` instance. Returns a teardown function for hot reloading or application exit.

#### `resolveHostPreloadPath(): string`

Returns the absolute path to `@open-snapora/electron`'s built-in preload script.

---

### Preload Exports (`@open-snapora/electron/preload`)

#### `exposeScreenshotApi(options: ExposeScreenshotApiOptions): ScreenshotRendererApi`

Binds `capture()` and `cancel()` methods into the renderer `window` context safely via `contextBridge`.

---

### Type Definitions

#### `ScreenshotOptions`

```typescript
export interface ScreenshotOptions {
  /** Target monitor: 'cursor' (default), 'primary', or specific display ID */
  display?: 'cursor' | 'primary' | string;

  /** Visible annotation tools */
  tools?: ScreenshotTool[];

  /** Default tool selected on open */
  defaultTool?: 'select' | ScreenshotTool;

  /** Show toast prompt when copied to clipboard (default: false) */
  showCopyFeedback?: boolean;

  /** Localization: 'en-US' | 'zh-CN' | 'ja-JP' | 'ko-KR' | 'es-ES' */
  locale?: ScreenshotLocale;

  /** Text overrides for buttons and tooltips */
  messages?: ScreenshotMessageOverrides;

  /** Visual theme overrides */
  theme?: ScreenshotTheme;
}
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
  bounds: { x: number; y: number; width: number; height: number };
  displayId: string;
}

export type ScreenshotOutputMetadata =
  | { action: 'copy' }
  | { action: 'save'; filePath: string }
  | { action: 'pin' };
```

---

## Advanced Recipes

### 1. Custom Cloud Upload Output (`outputAdapter`)

Intercept the output pipeline to upload snapshots to your cloud storage directly:

```typescript
import { ScreenshotManager, registerScreenshotIpc } from '@open-snapora/electron';

const manager = new ScreenshotManager({
  outputAdapter: {
    execute: async (payload, context) => {
      if (payload.action === 'save') {
        // Upload to S3/OSS instead of local disk
        const url = await uploadToCloud(payload.result.data);
        return {
          status: 'completed',
          output: { action: 'save', filePath: url },
        };
      }
      // Fallback to default copy/pin behaviour
      return null;
    },
  },
});
```

### 2. Injecting Native OS Window Bounds for Snapping

By default, Snapora snaps to visible `BrowserWindow` instances inside the current Electron process. To snap to third-party native OS windows, provide `getWindowSnapRegions`:

```typescript
const snapora = setupElectronSnapora({
  managerOptions: {
    getWindowSnapRegions: () => {
      // Query native windows via native addon or OS API
      return [
        { x: 0, y: 0, width: 1920, height: 1080 },
        { x: 100, y: 100, width: 800, height: 600 },
      ];
    },
  },
});
```

### 3. Telemetry & Performance Diagnostics

Track exact millisecond timings across every capture stage:

```typescript
const snapora = setupElectronSnapora({
  managerOptions: {
    onDiagnostic: (event) => {
      console.log(`[Snapora:${event.stage}:${event.phase}] in ${event.durationMs ?? 0}ms`, event);
    },
  },
});
```

---

## Example Demo

A complete Electron reference application is available in the [`demos/electron`](https://github.com/open-toolkits/open-snapora/tree/main/demos/electron) directory.

---

## License

MIT © [open-snapora authors](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
